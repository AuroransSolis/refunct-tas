use native::ue::*;
use native::actor::{AActor, EAutoReceiveInputType};
use native::controller::AController;

#[repr(C)]
pub struct APawn<B> {
    pub base: AActor<B>,
    b_use_controller_rotation_pitch: Bool32,
    b_use_controller_rotation_yaw: Bool32,
    b_use_controller_rotation_roll: Bool32,
    b_can_affect_navigation_generation: Bool32,
    b_input_enabled: Bool32,
    base_eye_height: f32,
    auto_posses_player: EAutoReceiveInputType,
    auto_possess_ai: EAutoPossessAi,
    ai_controller_class: TSubclassOf<AController>,
    player_state: *const APlayerState,
    remote_view_pitch: u8,
    last_hit_by: *const AController,
    controller: *const AController,
    allowed_yaw_error: f32,
    b_processing_outside_world_bonuds: Bool32,
    control_input_vector: FVector,
    last_control_input_vector: FVector,
}

#[repr(u8)]
pub enum EAutoPossessAi {
    Disabled,
    PlacedInWorld,
    Spawned,
    PlacedInWorldOrSpawned,
}