use std::{collections::HashSet, env::consts::DLL_SUFFIX, ffi::CStr, fs::{self, ReadDir}, os::raw::c_char, panic::AssertUnwindSafe, path::{Path, PathBuf}};

use libloading::Library;

use crate::{errors::{mod_error::ModError, registry_error::RegistryError}, mod_registry::{core_mod::CORE_MOD_ID, engine::ENGINE_BUILD_ID, registry::{ModContext, ModManifest, ModRegistry}}};

struct LoadedMod {
    pub manifest: ModManifest,
    pub register: RegisterFn,
}

pub type RegisterFn = fn(&mut ModContext<'_>) -> Result<(), RegistryError>;

pub fn load_mods(dir: &Path, mod_registry: ModRegistry) -> Result<ModRegistry, ModError> {
    let mut mods = Vec::<LoadedMod>::new();
    read_mods(&mut mods, dir)?;
    process_mods(mods, mod_registry)
}

fn read_mods(mods: &mut Vec<LoadedMod>, dir: &Path) -> Result<(), ModError> {
    if let Ok(dir_entries)  = fs::read_dir(dir) {
        collect_mods(mods, dir_entries)
    } else {
        Ok(())
    }
}

fn collect_mods(mods: &mut Vec<LoadedMod>, dir_entries: ReadDir) -> Result<(), ModError> {
    for entry in dir_entries {
        let path = entry.unwrap().path();
        if !path.to_string_lossy().ends_with(DLL_SUFFIX) { continue; }
        let lib: &'static Library = Box::leak(Box::new(unsafe { Library::new(&path).unwrap() }));
        
        let build_id: libloading::Symbol<unsafe extern "C" fn() -> *const c_char> =
            unsafe { lib.get(b"mr_build_id").unwrap() };
        let theirs = unsafe { CStr::from_ptr(build_id()) };
        if theirs != ENGINE_BUILD_ID {
            return Err(ModError::BuildMismatch { path, 
                expected: ENGINE_BUILD_ID.to_string_lossy().into(), found: theirs.to_string_lossy().into() });
        }

        let manifest_fn: libloading::Symbol<fn() -> ModManifest> =
            unsafe { lib.get(b"mr_manifest").unwrap() };
        let register_fn: libloading::Symbol<RegisterFn> =
            unsafe { lib.get(b"mr_register").unwrap() };

        mods.push(LoadedMod {
            manifest: manifest_fn(),
            register: *register_fn,
        });
    }

    Ok(())
}

fn process_mods(mods: Vec<LoadedMod>, mut mod_registry: ModRegistry) -> Result<ModRegistry, ModError> {
    let mods = topological_sort(mods)?;
    assert_unique(&mods)?;
    register_mods(&mods, &mut mod_registry)?;
    
    Ok(mod_registry)
}

fn assert_unique(mods: &[LoadedMod]) -> Result<(), ModError> {
    mods.iter()
        .map(|m| m.manifest.mod_id)
        .fold(Ok(CORE_MOD_ID), {|res, b|
            if res.is_ok_and(|a| a != b) {
                Ok(b)
            } else if b == CORE_MOD_ID {
                Err(ModError::ReservedModID { id: b.to_string() })
            } else {
                Err(ModError::DuplicateModID { id: b.to_string() })
            }
        })?;

    Ok(())
}

fn register_mods(mods: &Vec<LoadedMod>, mod_registry: &mut ModRegistry) -> Result<(), ModError> {
    for loaded in mods {
        let manifest = &loaded.manifest;
        let register = loaded.register;

        let mut ctx = mod_registry.build_context(manifest.mod_id);
        let result = std::panic::catch_unwind(AssertUnwindSafe(|| (register)(&mut ctx)));

        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => return Err(ModError::Registration { id: manifest.mod_id.to_string(), source: Box::new(e) }),
            Err(_)     => return Err(ModError::Panicked { id: manifest.mod_id.to_string() }),
        }
    }

    Ok(())
}

fn topological_sort(mut pending: Vec<LoadedMod>) -> Result<Vec<LoadedMod>, ModError> {
    pending.sort_by(|a, b| a.manifest.mod_id.cmp(b.manifest.mod_id));

    let mut done: HashSet<&str> = HashSet::from([CORE_MOD_ID]);
    let mut sorted = Vec::with_capacity(pending.len());

    while !pending.is_empty() {
        let ready: Vec<LoadedMod> = pending
            .extract_if(.., |m| m.manifest.dependencies.iter().all(|d| done.contains(d)))
            .collect();

        if ready.is_empty() {
            let ids = pending.into_iter().map(|m| m.manifest.mod_id.to_string()).collect();
            return Err(ModError::UnresolvedDependencies { ids });
        }

        for m in ready {
            done.insert(m.manifest.mod_id);
            sorted.push(m);
        }
    }

    Ok(sorted)
}

pub fn mods_dir() -> PathBuf {
    std::env::current_exe()
        .expect("couldn't locate the running executable")
        .parent()
        .expect("executable has no parent directory")
        .join("mods")
}

#[cfg(test)]
mod tests {
    use crate::{components::test_util::TestStatelessComponent, mod_registry::{load_mods::{LoadedMod, process_mods}, registry::ModRegistry, test_util::{CIRCULAR_MANIFEST1, CIRCULAR_MANIFEST2, DUPLICATE_MANIFEST, RESERVED_MANIFEST, TEST_MANIFEST1, TEST_MANIFEST2, register_duplicate_tag, register_duplicate_type, register_noop, register_panic, register_test}}};
    use crate::mod_registry::mod_element::ModElement;

    #[test]
    fn process_one() {
        let mut mods: Vec<LoadedMod> = Vec::new();
        let mod_registry = ModRegistry::new();

        mods.push(LoadedMod {
            manifest: TEST_MANIFEST1,
            register: register_test,
        });

        let mod_registry = process_mods(mods, mod_registry).unwrap();

        let tags: Vec<_> = mod_registry.get_tags().cloned().collect();
        let tag = format!("{}:{}", TEST_MANIFEST1.mod_id, TestStatelessComponent::NAME);

        assert!(tags.contains(&tag));
    }

    #[test]
    fn process_chain() {
        let mut mods: Vec<LoadedMod> = Vec::new();
        let mod_registry = ModRegistry::new();

        mods.push(LoadedMod {
            manifest: TEST_MANIFEST2,
            register: register_test,
        });

        mods.push(LoadedMod {
            manifest: TEST_MANIFEST1,
            register: register_noop,
        });

        let mod_registry = process_mods(mods, mod_registry).unwrap();

        let tags: Vec<_> = mod_registry.get_tags().cloned().collect();
        let tag = format!("{}:{}", TEST_MANIFEST2.mod_id, TestStatelessComponent::NAME);

        assert!(tags.contains(&tag));
    }

    #[test]
    fn absent_dependencies() {
        let mut mods: Vec<LoadedMod> = Vec::new();
        let mod_registry = ModRegistry::new();

        mods.push(LoadedMod {
            manifest: CIRCULAR_MANIFEST1,
            register: register_noop,
        });

        let err = process_mods(mods, mod_registry).unwrap_err();

        println!("{err}")
    }

    #[test]
    fn reserved_manifest() {
        let mut mods: Vec<LoadedMod> = Vec::new();
        let mod_registry = ModRegistry::new();

        mods.push(LoadedMod {
            manifest: RESERVED_MANIFEST,
            register: register_noop,
        });

        let err = process_mods(mods, mod_registry).unwrap_err();

        println!("{err}")
    }

    #[test]
    fn duplicate_manifest() {
        let mut mods: Vec<LoadedMod> = Vec::new();
        let mod_registry = ModRegistry::new();

        mods.push(LoadedMod {
            manifest: TEST_MANIFEST1,
            register: register_noop,
        });
        mods.push(LoadedMod {
            manifest: DUPLICATE_MANIFEST,
            register: register_noop,
        });

        let err = process_mods(mods, mod_registry).unwrap_err();

        println!("{err}")
    }

    #[test]
    fn circular_dependencies() {
        let mut mods: Vec<LoadedMod> = Vec::new();
        let mod_registry = ModRegistry::new();

        mods.push(LoadedMod {
            manifest: CIRCULAR_MANIFEST1,
            register: register_noop,
        });
        mods.push(LoadedMod {
            manifest: CIRCULAR_MANIFEST2,
            register: register_noop,
        });

        let err = process_mods(mods, mod_registry).unwrap_err();

        println!("{err}")
    }

    #[test]
    fn duplicate_type() {
        let mut mods: Vec<LoadedMod> = Vec::new();
        let mod_registry = ModRegistry::new();

        mods.push(LoadedMod {
            manifest: TEST_MANIFEST1,
            register: register_duplicate_type,
        });

        let err = process_mods(mods, mod_registry).unwrap_err();

        println!("{err}")
    }

    #[test]
    fn duplicate_tag() {
        let mut mods: Vec<LoadedMod> = Vec::new();
        let mod_registry = ModRegistry::new();

        mods.push(LoadedMod {
            manifest: TEST_MANIFEST1,
            register: register_duplicate_tag,
        });

        let err = process_mods(mods, mod_registry).unwrap_err();

        println!("{err}")
    }

    #[test]
    fn doki_doki_panic() {
        let mut mods: Vec<LoadedMod> = Vec::new();
        let mod_registry = ModRegistry::new();

        mods.push(LoadedMod {
            manifest: TEST_MANIFEST1,
            register: register_panic,
        });

        let err = process_mods(mods, mod_registry).unwrap_err();

        println!("{err}")
    }
}