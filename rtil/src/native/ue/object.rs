use native::ue::*;

#[repr(C)]
pub struct UObjectBase<B> {
    vtable: *const (), // 0x000
    object_flags: EObjectFlags, // 0x008
    internal_index: i32, // 0x00c
    class_private: *const UClass, // 0x0x010
    name_private: FName, // 0x018 / 0x010
    outer_private: *const B, // 0x020 / 0x018
    vtable2: *const (), // 0x028 / 0x01c
} // 0x030 / 0x020

#[repr(C)]
pub struct UObject<B> {
    base: UObjectBase<B>
} // 0x030 / 0x020
