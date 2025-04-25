
use std::ffi::CString;
use std::os::raw::c_char;

// Just for testing purpose.
unsafe extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn printf(format: *const c_char, ...) -> i32;
    fn scanf(format: *const c_char, ...) -> i32;
}

fn rmalloc() {
    let ptr: *mut u64 = unsafe {
        malloc(100).cast()
    };

    unsafe {
        printf(CString::new("Value before = %d\n").unwrap().as_ptr(), ptr.read());
        ptr.write(2000);
        printf(CString::new("Value after = %d\n").unwrap().as_ptr(), ptr.read());
        let new_ptr: *mut u64 = ptr.add(1);
        scanf(CString::new("%lu").unwrap().as_ptr(), new_ptr);
        printf(CString::new("Value after reading value = %lu\n").unwrap().as_ptr(), new_ptr.read());
    }
}

#[cfg(test)]
mod tests {
    use super::rmalloc;

    #[test]
    fn test_malloc() {
        rmalloc();
    }
}