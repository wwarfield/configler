use std::{env, ffi::{CStr, CString}};

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


#[no_mangle]
pub extern "C" fn double(x: i32) -> i32 {
    x * 2
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