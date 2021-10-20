use std::alloc::{alloc, Layout};

/// Handles an output pointer by writing a value to it if the pointer
/// is not NULL.
///
/// If the pointer currently has a value stored in it, then the previous
/// value is properly deallocated. This assumes the value has not already
/// been freed (to avoid a double free).
///
/// Allows memory allocations to be reused over time.
pub unsafe fn handle_output_ptr<T>(out_ptr: *mut *mut T, out_value: T)
where
    T: Sized,
{
    // If output pointer is null, result is ignored and dropped.
    if !out_ptr.is_null() {
        if (*out_ptr).is_null() {
            // Initialize memory for the caller.
            *out_ptr = alloc(Layout::new::<T>()) as *mut T;
        } else {
            // Drop previous value, keeping allocation.
            Box::from_raw(*out_ptr);
        }
        // Write into existing allocation.
        std::ptr::write(*out_ptr, out_value);
    }
}
