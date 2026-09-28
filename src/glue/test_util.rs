use crate::glue::mod_registry::ModManifest;

pub const TEST_MANIFEST: ModManifest = ModManifest {
    mod_id: "test",
    version: "0.1.0",
    dependencies: &["core"]
};

pub const CIRCULAR_MANIFEST1: ModManifest = ModManifest {
    mod_id: "circular1",
    version: "0.1.0",
    dependencies: &["circular2"]
};

pub const CIRCULAR_MANIFEST2: ModManifest = ModManifest {
    mod_id: "circular2",
    version: "0.1.0",
    dependencies: &["circular1"]
};

//pub fn noop_register_fn()