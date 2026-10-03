use std::ffi::CStr;

pub const ENGINE_BUILD_ID: &CStr = match CStr::from_bytes_with_nul(
    concat!(env!("MR_ENGINE_BUILD_ID"), "\0").as_bytes(),
) {
    Ok(id) => id,
    Err(_) => panic!("engine build id must not contain a nul byte"),
};