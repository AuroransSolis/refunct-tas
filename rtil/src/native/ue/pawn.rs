use native::ue::*;

#[repr(C)]
pub struct APawn<B> {
    pub base: AActor<B>, // 0x000
    // TODO: some weird padding maybe?
    _pad: [Unk8; 2],
//    b_use_controller_rotation_pitch: Bool32,
//    b_use_controller_rotation_yaw: Bool32,
//    b_use_controller_rotation_roll: Bool32,
//    b_can_affect_navigation_generation: Bool32,
//    b_input_enabled: Bool32,
    bitfield: Bool32, // 0x390
    base_eye_height: f32, // 0x394
    auto_posses_player: TEnumAsByte<EAutoReceiveInputType>, // 0x398
    auto_possess_ai: EAutoPossessAi, // 0x399
    ai_controller_class: TSubclassOf<AController>, // 0x3a0
    player_state: *const APlayerState, // 0x3a8
    remote_view_pitch: u8, // 0x3b0
    last_hit_by: *const AController, // 0x3b8
    pub controller: *const AController, // 0x3c0
    allowed_yaw_error: f32, // 0x3c8
//    b_processing_outside_world_bounds: Bool32,
    bitfield2: Bool32, // 0x3cc
    control_input_vector: FVector, // 0x3d0
    last_control_input_vector: FVector, // 0x3dc
} // 0x3e8
