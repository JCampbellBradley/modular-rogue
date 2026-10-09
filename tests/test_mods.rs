use std::{env::consts::{DLL_PREFIX, DLL_SUFFIX}, fs, path::PathBuf};
use modular_rogue::mod_registry::{load_mods::load_mods, registry::ModRegistry};

fn build_dir() -> PathBuf {
    std::env::current_exe().unwrap()
        .parent().unwrap()
        .parent().unwrap()
        .to_path_buf()
}

fn stage_mods(test_name: &str, mods: &[&str]) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("mr_{test_name}_{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();

    for name in mods {
        let file = format!("{DLL_PREFIX}{name}{DLL_SUFFIX}");
        let src = build_dir().join(&file);
        assert!(src.exists(), "{} missing; build it with `cargo build -p {}`", src.display(), name);
        fs::copy(&src, dir.join(&file)).unwrap();
    }
    dir
}

#[test]
#[ignore = "needs fixture mods built; run via `cargo xtask test-mods`"]
fn basic_mod() {
    let dir = stage_mods("loads_blank_mod", &["blank_mod"]);
    let _registry = load_mods(&dir, ModRegistry::new()).unwrap();
}

#[test]
#[ignore = "needs fixture mods built; run via `cargo xtask test-mods`"]
fn minimal_mod() {
    let dir = stage_mods("loads_minimal_mod", &["minimal_mod"]);
    let registry = load_mods(&dir, ModRegistry::new()).unwrap();

    assert!(registry.get_tags().any(|tag| tag == "minimal_mod:test_component"))
}