use native::ue::*;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FRotator {
    pitch: f32,
    yaw: f32,
    roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[allow(non_camel_case_types)]
pub type FVector_NetQuantize10 = FVector;
#[allow(non_camel_case_types)]
pub type FVector_NetQuantize100 = FVector;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct FVector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FIntVector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FQuat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FTransform {
    pub rotation: FQuat,
    pub translation: FVector,
    pub scale3d: FVector,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FVector2D {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FRotationConversionCache {
    cached_quat: FQuat,
    cached_rotator: FRotator,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FBox {
    min: FVector, // 0x000
    max: FVector, // 0x00c
    is_valid: Bool8, // 0x018
} // 0x019

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FBoxSphereBounds {
    origin: FVector, // 0x000
    box_extent: FVector, // 0x00c
    sphere_radius: f32, // 0x018
} // 0x01c
