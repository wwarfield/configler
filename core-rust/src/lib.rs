use std::{env, ffi::{CStr, CString}};
use pyo3::prelude::*;

#[repr(C)]
struct ConfigPOC {}

impl ConfigPOC {
    pub fn get_env_val(&self, name: &str, default: &str) -> String {
        env::var(name).unwrap_or(default.to_owned())
    }

    #[no_mangle]
    pub extern "C" fn c_get_env_val(&self, c_name: *const std::ffi::c_char, c_default: *const std::ffi::c_char) -> *const std::ffi::c_char {
        // note would probably require some error handling for non utf-8 strings
        let cstr_name = unsafe { CStr::from_ptr(c_name) };
        let name = cstr_name.to_str().unwrap();

        let cstr_default = unsafe { CStr::from_ptr(c_default) };
        let default = cstr_default.to_str().unwrap();
        // let name = CString::new(c_name)
        // let default = c_default.into_string().unwrap();

        let value = self.get_env_val(name, default);
        CString::new(value).unwrap().into_raw()
    }
}




/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn string_sum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}

// fn main() {
//     let config = ConfigPOC {};
//     println!(
//         "Hello, world! TEST VALUE {}",
//         config.get_env_val("TEST".to_string(), "default val".to_string())
//     );
// }

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }