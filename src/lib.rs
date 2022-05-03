#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::os::raw;
    use std::ptr;
    use crate::*;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        unsafe {
            let mut zip: *mut raw::c_void = ptr::null_mut();
            let ret = mz_zip_writer_create(&mut zip);
            if ret == ptr::null_mut() {
                panic!("mz_zip_writer_create failed");
            }

            let zip_filename = "/tmp/test.zip";
            let zip_filename = CString::new(zip_filename).unwrap();
            let ret = mz_zip_writer_open_file(
                zip, 
                zip_filename.as_ptr(),
                0, 
                0);
            assert_eq!(ret, MZ_OK);

            let password = "za warudo";
            let password = CString::new(password).unwrap();
            mz_zip_writer_set_password(zip, password.as_ptr());
            mz_zip_writer_set_aes(zip, 1);
            mz_zip_writer_set_compress_method(zip, MZ_COMPRESS_METHOD_STORE as u16);

            let src_file = "/etc/profile";
            let src_file = CString::new(src_file).unwrap();
            let entry_name = "profile";
            let entry_name = CString::new(entry_name).unwrap();
            let ret = mz_zip_writer_add_file(zip, src_file.as_ptr(), entry_name.as_ptr());
            assert_eq!(ret, MZ_OK);

            let ret = mz_zip_writer_close(zip);
            assert_eq!(ret, MZ_OK);

            mz_zip_writer_delete(&mut zip);
        }
    }
}
