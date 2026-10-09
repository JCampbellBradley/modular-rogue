use std::{env::consts::DLL_SUFFIX, ffi::CStr, fs::ReadDir, os::raw::c_char, path::PathBuf};

use libloading::Library;
use modular_rogue::{errors::mod_error::ModError, mod_registry::{engine::ENGINE_BUILD_ID, load_mods::{LoadedMod, RegisterFn}, registry::ModManifest}};


pub fn mods_dir() -> PathBuf {
    std::env::current_exe()
        .expect("couldn't locate the running executable")
        .parent()
        .expect("executable has no parent directory")
        .join("mods")
}

pub fn collect_mods(mods: &mut Vec<LoadedMod>, dir_entries: ReadDir) -> Result<(), ModError> {
    for entry in dir_entries {
        let path = entry?.path();
        if !path.to_string_lossy().ends_with(DLL_SUFFIX) { continue; }
        let lib: &'static Library = Box::leak(Box::new(unsafe { Library::new(&path)? }));
        
        let build_id: libloading::Symbol<unsafe extern "C" fn() -> *const c_char> =
            unsafe { lib.get(b"mr_build_id")? };
        let theirs = unsafe { CStr::from_ptr(build_id()) };
        if theirs != ENGINE_BUILD_ID {
            return Err(ModError::BuildMismatch { path, 
                expected: ENGINE_BUILD_ID.to_string_lossy().into(), found: theirs.to_string_lossy().into() });
        }

        let manifest_fn: libloading::Symbol<fn() -> ModManifest> =
            unsafe { lib.get(b"mr_manifest")? };
        let register_fn: libloading::Symbol<RegisterFn> =
            unsafe { lib.get(b"mr_register")? };

        mods.push(LoadedMod {
            manifest: manifest_fn(),
            register: *register_fn,
        });
    }

    Ok(())
}