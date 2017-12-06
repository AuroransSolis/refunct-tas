use std::ptr;
use std::mem;
use std::marker::PhantomData;
use std::ops::Index;

use native::ue::*;
use native::FNAME_FNAME;

#[repr(C)]
#[derive(Debug)]
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

    pub fn capacity(&self) -> i32 {
        self.capacity
    }
    pub fn len(&self) -> i32 {
        self.len
    }
}

impl<T> Index<usize> for TArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        assert!((index as i32) < self.len, "index out of range");
        unsafe {
            &*self.ptr.offset(index as isize)
        }
    }
}

impl<T> Index<i32> for TArray<T> {
    type Output = T;

    fn index(&self, index: i32) -> &T {
        assert!(index >= 0, "negative index");
        self.index(index as usize)
    }
}

/// Null-terminated utf-32 array
pub struct FString(TArray<char>);

impl FString {
    pub fn new() -> FString {
        FString(TArray::new())
    }

    pub unsafe fn as_ptr(&self) -> *const char {
        self.0.ptr
    }
}

impl<S: AsRef<str>> From<S> for FString {
    fn from(s: S) -> Self {
        assert_eq!(::std::mem::size_of::<char>(), 4);
        let s = s.as_ref();
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

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FName {
    number: u64,
}

impl<T: Into<FString>> From<T> for FName {
    fn from(s: T) -> FName {
        let s = s.into();
        let mut name = FName {
            number: 0,
        };
        unsafe {
            let fun: extern "C" fn(*mut FName, *const char, u64) -> u64
            = mem::transmute(FNAME_FNAME);
            fun(&mut name as *mut FName, s.as_ptr(), 1);
        }
        name
    }
}

#[repr(C)]
pub struct TLinkedList<T> {
    next_link: *const T,
    prev_link: *const *const T,
    element: T,
}
