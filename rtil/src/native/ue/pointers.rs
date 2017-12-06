use std::marker::PhantomData;

use native::ue::*;

#[repr(C)]
pub struct TWeakObjectPtr<T> {
    object_index: i32,
    object_serial_number: i32,
    _t: PhantomData<T>,
}

#[repr(C)]
pub struct TSharedPtr<T> {
    ptr: *const T,
    shared_reference_count: *const FReferenceControllerBase,
}

#[repr(C)]
pub struct TSubclassOf<C> {
    class: *const UClass,
    _c: PhantomData<C>,
}

/// Implements ref counting for classes implementing AddRef / Release
#[repr(C)]
pub struct TRefCountPtr<T> {
    reference: *const T,
}

#[repr(C)]
pub struct TSharedRef<T> {
    object: *const T,
}

pub type FGraphEventRef = TRefCountPtr<FGraphEvent>;
