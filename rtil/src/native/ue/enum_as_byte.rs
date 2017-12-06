use std::marker::PhantomData;

#[repr(C)]
pub struct TEnumAsByte<T> {
    value: u8,
    _t: PhantomData<T>,
}
