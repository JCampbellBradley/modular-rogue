use std::{env, fs, hash::{DefaultHasher, Hash, Hasher}, path::Path, process::Command};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("linux") {
        println!("cargo::rustc-link-arg-bins=-Wl,-rpath,$ORIGIN");
    }

    let rustc = env::var("RUSTC").expect("cargo sets RUSTC for build scripts");
    let out = Command::new(&rustc).arg("-V").output().expect("failed to run rustc");
    let rustc_version = String::from_utf8(out.stdout).unwrap().trim().to_owned();

    let mut hasher = DefaultHasher::new();
    for input in ["src", "modular-rogue-macros/src", "Cargo.toml", "Cargo.lock"] {
        hash_path(Path::new(input), &mut hasher);
        println!("cargo::rerun-if-changed={input}");
    }
    env::var("CARGO_ENCODED_RUSTFLAGS").unwrap_or_default().hash(&mut hasher);

    let build_id = format!(
        "{} | {} | {} | {} | {:016x}",
        env::var("CARGO_PKG_VERSION").unwrap(),
        rustc_version,
        env::var("TARGET").unwrap(),
        env::var("PROFILE").unwrap(),
        hasher.finish(),
    );

    println!("cargo::rustc-env=MR_ENGINE_BUILD_ID={build_id}");
}

fn hash_path(path: &Path, hasher: &mut DefaultHasher) {
    if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path).unwrap()
            .map(|e| e.unwrap().path())
            .collect();
        entries.sort(); // read_dir order is arbitrary; sorting
        for entry in entries {
            hash_path(&entry, hasher);
        }
    } else {
        path.hash(hasher);
        fs::read(path).unwrap().hash(hasher);
    }
}