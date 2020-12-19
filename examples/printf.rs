use std::os::raw::{c_char, c_int};

extern "C" {
    // three dot `...` bind C Arbitrary Arguments
    fn printf(format: *const c_char, ...) -> c_int;
}

fn main() {
    let a: i32 = 1;
    unsafe {
        printf("a = %d\n".as_ptr() as *const _, a);
    }
}
