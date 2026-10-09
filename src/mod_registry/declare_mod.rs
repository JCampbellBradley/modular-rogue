#[macro_export]
macro_rules! declare_mod {
    (
        id: $id:literal,
        version: $version:literal,
        depends: [$($dep:literal),* $(,)?]
    ) => {
        #[$crate::linkme::distributed_slice]
        #[linkme(crate = $crate::linkme)]
        pub static MR_COMPONENTS: [$crate::mod_registry::load_mods::RegisterFn];

        #[unsafe(no_mangle)]
        pub extern "C" fn mr_build_id() -> *const ::std::os::raw::c_char {
            $crate::mod_registry::engine::ENGINE_BUILD_ID.as_ptr()
        }

        #[unsafe(no_mangle)]
        pub fn mr_manifest() -> $crate::mod_registry::registry::ModManifest {
            $crate::mod_registry::registry::ModManifest {
                mod_id: $id,
                version: $version,
                dependencies: &[$($dep),*],
            }
        }

        #[unsafe(no_mangle)]
        pub fn mr_register(
            ctx: &mut $crate::mod_registry::registry::ModContext<'_>,
        ) -> Result<(), $crate::errors::registry_error::RegistryError> {
            for f in MR_COMPONENTS { f(ctx)?; }
            Ok(())
        }
    };
}