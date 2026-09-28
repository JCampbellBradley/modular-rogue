use crate::{components::test_util::{DuplicateTagComponent, TestZeroHandlesComponent}, errors::registry_error::RegistryError, glue::mod_registry::{ModContext, ModManifest}};

pub const TEST_MANIFEST1: ModManifest = ModManifest {
    mod_id: "test1",
    version: "0.1.0",
    dependencies: &["core"]
};

pub const DUPLICATE_MANIFEST: ModManifest = ModManifest {
    mod_id: "test1",
    version: "0.1.0",
    dependencies: &["core"]
};

pub const TEST_MANIFEST2: ModManifest = ModManifest {
    mod_id: "test2",
    version: "0.1.0",
    dependencies: &["test1"]
};

pub const RESERVED_MANIFEST: ModManifest = ModManifest {
    mod_id: "core",
    version: "0.1.0",
    dependencies: &[]
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

pub fn register_noop(_ctx: &mut ModContext<'_>) -> Result<(), RegistryError> {
    Ok(())
}

pub fn register_test(ctx: &mut ModContext<'_>) -> Result<(), RegistryError> {
    ctx.register_component::<TestZeroHandlesComponent>()?;

    Ok(())
}

pub fn register_duplicate_type(ctx: &mut ModContext<'_>) -> Result<(), RegistryError> {
    ctx.register_component::<TestZeroHandlesComponent>()?;
    ctx.register_component::<TestZeroHandlesComponent>()?;

    Ok(())
}

pub fn register_duplicate_tag(ctx: &mut ModContext<'_>) -> Result<(), RegistryError> {
    ctx.register_component::<TestZeroHandlesComponent>()?;
    ctx.register_component::<DuplicateTagComponent>()?;

    Ok(())
}

pub fn register_panic(_ctx: &mut ModContext<'_>) -> Result<(), RegistryError> {
    panic!()
}