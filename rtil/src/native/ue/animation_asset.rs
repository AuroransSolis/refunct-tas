use native::ue::*;

#[repr(C)]
pub struct FRootMotionMovementParams {
    b_has_root_motion: bool,
    blend_weight: bool,
    root_motion_transform: FTransform,
}

