#[linkme::distributed_slice]
pub static MR_COMPONENTS: [crate::mod_registry::load_mods::RegisterFn];

pub static CORE_MOD_ID: &str = "core";

pub fn mr_register(ctx: &mut crate::mod_registry::registry::ModContext<'_>,) -> Result<(), crate::errors::registry_error::RegistryError>{
    for f in MR_COMPONENTS {
        f(ctx)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{components::stats::characteristic_stats::CharacteristicStats, mod_registry::{mod_element::ModElement, registry::ModRegistry}};

    use super::*;

    #[test]
    fn register() {
        let mut reg = ModRegistry::new();
        let mut ctx = reg.build_context(CORE_MOD_ID);

        mr_register(&mut ctx).unwrap();

        let tag = format!("{CORE_MOD_ID}:{}", CharacteristicStats::NAME);
        assert!(reg.get_tags().any(|t| *t == tag));
    }
}