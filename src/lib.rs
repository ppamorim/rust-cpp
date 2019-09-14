#[macro_use]
extern crate cpp;

cpp!{{
    #include <iostream>
    #include "sum.h"
}}

pub fn foo() {
    let name = std::ffi::CString::new("World").unwrap();
    let name_ptr = name.as_ptr();
    let r = unsafe {
        cpp!([name_ptr as "const char *"] -> u32 as "int32_t" {
            std::cout << "Hello, " << name_ptr << std::endl;
            int r = sum(3);
            return r;
        })
    };
    assert_eq!(r, 4)
}