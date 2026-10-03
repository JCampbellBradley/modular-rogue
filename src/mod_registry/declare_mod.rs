#[macro_export]
macro_rules! declare_mod {
    (
        id: $id:literal,
        version: $version:literal,
        depends: [$($dep:literal),* $(,)?]
    ) => {
        #[linkme::distributed_slice]
        pub static MR_COMPONENTS: [$crate::glue::load_mods::RegisterFn];

        #[unsafe(no_mangle)]
        pub extern "C" fn mr_build_id() -> *const ::std::os::raw::c_char {
            $crate::glue::engine::ENGINE_BUILD_ID.as_ptr()
        }

        #[unsafe(no_mangle)]
        pub fn mr_manifest() -> $crate::glue::mod_registry::ModManifest {
            $crate::glue::mod_registry::ModManifest {
                mod_id: $id.to_string(),
                version: $version.to_string(),
                dependencies: vec![$($dep.to_string()),*],
            }
        }

        #[unsafe(no_mangle)]
        pub fn mr_register(
            ctx: &mut $crate::glue::mod_registry::ModContext<'_>,
        ) -> Result<(), $crate::errors::registry_error::RegistryError> {
            for f in MR_COMPONENTS { f(ctx)?; }
            Ok(())
        }
    };
}