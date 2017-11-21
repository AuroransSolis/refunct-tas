use std::ptr;
use std::mem;

use native::FMemory;

#[repr(C, packed)]
pub struct FVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C, packed)]
pub struct TArray<T> {
    ptr: *mut T,
    len: i32,
    capacity: i32,
}

impl<T> TArray<T> {
    pub fn new() -> TArray<T> {
        TArray::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> TArray<T> {
        let ptr = if cap > 0 {
            unsafe { FMemory::malloc(cap * mem::size_of::<T>()) as *mut T }
        } else {
            ptr::null_mut()
        };
        TArray {
            ptr,
            len: 0,
            capacity: cap as i32,
        }
    }

    pub fn push(&mut self, t: T) {
        assert!(self.len  < self.capacity);
        unsafe { *self.ptr.offset(self.len as isize) = t };
        self.len += 1;
    }
}

/// Null-terminated utf-32 array
pub struct FString(TArray<char>);

impl FString {
    pub fn new() -> FString {
        FString(TArray::new())
    }
}

impl<'a> From<&'a str> for FString {
    fn from(s: &'a str) -> Self {
        assert_eq!(::std::mem::size_of::<char>(), 4);
        let len = s.chars().count();
        let mut arr = TArray::with_capacity(len + 1);
        for c in s.chars() {
            arr.push(c)
        }
        arr.push(0 as char);

        FString(arr)
    }
}

impl<T> Drop for TArray<T> {
    fn drop(&mut self) {
        unsafe {
            FMemory::free(self.ptr as *mut ())
        }
    }
}
