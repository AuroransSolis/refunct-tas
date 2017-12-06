use native::ue::*;

#[repr(C)]
pub struct FRootMotionSourceGroup {
    root_motion_sources: TArray<TSharedPtr<FRootMotionSource>>,
    pending_add_root_moution_sources: TArray<TSharedPtr<FRootMotionSource>>,
    b_has_additive_sources: bool,
    b_has_override_sources: bool,
    last_pre_additive_velocity: FVector_NetQuantize10,
    b_is_additive_velocity_applied: bool,
    last_accumulated_settings: FRootMotionSourceSettings,
}

#[repr(C)]
pub struct FRootMotionSourceSettings {
    flags: u8,
}

#[repr(C)]
pub struct FRootMotionServerToLocalIdMapping {
    server_id: u16,
    local_id: u16,
    time_stamp: f32,
}

