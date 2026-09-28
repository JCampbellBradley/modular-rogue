use std::{env::consts::{DLL_PREFIX, DLL_SUFFIX}, fs, path::{Path, PathBuf}, process::Command};

fn main() {
    let status = Command::new(env!("CARGO"))
        .args(["build", "--release", "--workspace"])
        .status()
        .expect("failed to run cargo");
    assert!(status.success());


    assemble_dist(&[]);
}

fn assemble_dist(mods: &[&str]) {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();

    let mods_dir = root_dir.join("target/release/mods");
    fs::create_dir_all(mods_dir).unwrap();

    let release = root_dir.join(Path::new("target/release"));
    let dist = root_dir.join(Path::new("dist"));
    let _ = fs::remove_dir_all(&dist);
    fs::create_dir_all(&dist.join("mods")).unwrap();

    let exe = format!("modular-rogue-game{}", std::env::consts::EXE_SUFFIX);
    let engine = format!("{DLL_PREFIX}modular_rogue{DLL_SUFFIX}");
    fs::copy(release.join(&exe), dist.join(&exe)).unwrap();
    fs::copy(release.join(&engine), dist.join(&engine)).unwrap();

    let std_lib = find_libstd();
    fs::copy(&std_lib, dist.join(std_lib.file_name().unwrap())).unwrap();

    for name in mods {
        let file = format!("{DLL_PREFIX}{name}{DLL_SUFFIX}");
        fs::copy(release.join(&file), dist.join("mods").join(&file)).unwrap();
    }
}

fn rustc_output(args: &[&str]) -> String {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".into());
    let out = Command::new(rustc).args(args).output().expect("failed to run rustc");
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap().trim().to_owned()
}

fn find_libstd() -> PathBuf {
    let sysroot = PathBuf::from(rustc_output(&["--print", "sysroot"]));
    let host = rustc_output(&["-vV"])
        .lines()
        .find_map(|l| l.strip_prefix("host: "))
        .expect("no host line in rustc -vV")
        .to_owned();

    let prefix = format!("{DLL_PREFIX}std-");
    let dirs = [sysroot.join("lib/rustlib").join(&host).join("lib"), sysroot.join("bin")];

    let mut found: Vec<PathBuf> = dirs.iter()
        .filter_map(|d| fs::read_dir(d).ok())
        .flatten()
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| p.file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|n| n.starts_with(&prefix) && n.ends_with(DLL_SUFFIX)))
        .collect();

    assert_eq!(found.len(), 1, "expected exactly one std library, found {found:?}");
    found.pop().unwrap()
}