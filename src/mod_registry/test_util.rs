use crate::{components::test_util::{DuplicateTagComponent, TestStatefulComponent, TestStatelessComponent}, errors::{expect_messages::TEST_UNREACHABLE, registry_error::RegistryError}, mod_registry::registry::{ModContext, ModManifest, ModRegistry}};

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
    ctx.register_component::<TestStatelessComponent>()
}

pub fn register_duplicate_type(ctx: &mut ModContext<'_>) -> Result<(), RegistryError> {
    ctx.register_component::<TestStatelessComponent>().expect(TEST_UNREACHABLE);
    ctx.register_component::<TestStatelessComponent>()
}

pub fn register_duplicate_tag(ctx: &mut ModContext<'_>) -> Result<(), RegistryError> {
    ctx.register_component::<TestStatelessComponent>().expect(TEST_UNREACHABLE);
    ctx.register_component::<DuplicateTagComponent>()
}

pub fn register_panic(_ctx: &mut ModContext<'_>) -> Result<(), RegistryError> {
    panic!()
}

pub fn reg_with_test_components() -> ModRegistry {
    let mut reg = ModRegistry::new();
    let mod_id = "test";
    let mut ctx = reg.build_context(mod_id);

    ctx.register_component::<TestStatelessComponent>().unwrap();
    ctx.register_component::<TestStatefulComponent>().unwrap();

    reg
}