#[linkme::distributed_slice]
pub static MR_COMPONENTS: [crate::glue::load_mods::RegisterFn];

pub static CORE_MOD_ID: &str = "core";

pub fn mr_register(ctx: &mut crate::glue::mod_registry::ModContext<'_>,) -> Result<(), crate::errors::registry_error::RegistryError>{
    for f in MR_COMPONENTS {
        f(ctx)?;
    }
    Ok(())
}