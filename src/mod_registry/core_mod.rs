#[linkme::distributed_slice]
pub static MR_COMPONENTS: [crate::mod_registry::load_mods::RegisterFn];

pub static CORE_MOD_ID: &str = "core";

pub fn mr_register(ctx: &mut crate::mod_registry::registry::ModContext<'_>,) -> Result<(), crate::errors::registry_error::RegistryError>{
    for f in MR_COMPONENTS {
        f(ctx)?;
    }
    Ok(())
}