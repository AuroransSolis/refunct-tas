use native::ue::*;
use native::pawn::APawn;
use native::character::ACharacter;
use native::actor::AActor;
use native::level::ULevel;
use native::scenecomponent::USceneComponent;

use native::ACONTROLLER_GETCONTROLROTATION;
#[cfg(unix)] use native::linux::controller::save;
#[cfg(windows)] use native::windows::controller::save;

type FInstigatedAnyDamageSignature = *const ();

#[repr(C)]
pub struct AController {
    base: AActor<ULevel>, // 0x000
    _unk: Unk8, // 0x380
    _unk2: *const (), // 0x388
    pawn: *const APawn<()>, // 0x390
    old_pawn: TWeakObjectPtr<APawn<()>>, // 0x398
    character: *const ACharacter<()>, // 0x3a0
    player_state: *const APlayerState, // 0x3a8
    transform_component: *const USceneComponent, // 0x3b0
    control_rotation: FRotator, // 0x3b8
//    b_attach_to_pawn: Bool32,
//    b_is_player_controller: Bool32,
    bitfield: Bool32, // 0x3c4
    // not in memory (new?)
//    ignore_move_input: Bool8,
//    ignore_look_input: Bool8,
    start_spot: TWeakObjectPtr<AActor<()>>, // 0x3c8
    state_name: FName, // 0x3d0
    on_instigated_any_damage: FInstigatedAnyDamageSignature, // 0x3d8
} // 0x3e0

#[repr(C)]
pub struct APlayerController {
    base: AController, // 0x000
    // TODO: weird padding?
    _unk: Unk8, // 0x3e0
    player: *const UPlayer, // 0x3e8
//    b_short_connect_time_out: Bool32,
    bitfield: Bool32, // 0x3f0
    acknowledged_pawn: *const APawn<()>, // 0x3f8
    controlling_dir_track_inst: *const UInterpTrackInstDirector, // 0x400
    local_player_cached_lod_distance_factor: f32, // 0x408
    pub my_hud: *const AHud, // 0x410
    player_camera_manager: *const APlayerCameraManager, // 0x418
    player_camera_manager_class: TSubclassOf<APlayerCameraManager>,
    b_auto_manage_active_camera_target: bool,
    target_view_rotation: FRotator,
    blended_target_view_rotation: FRotator,
    smooth_target_view_rotation_speed: f32,
    hidden_actors: TArray<*const AActor<()>>,
    hidden_primitive_components: TArray<TWeakObjectPtr<UPrimitiveComponent>>,
    b_render_primitive_components: bool,
    last_spectator_state_synch_time: f32,
    last_spectator_synch_location: FVector,
    last_spectator_sync_rotation: FRotator,
    client_cap: i32,
    cheat_manager: *const UCheatManager,
    cheat_class: TSubclassOf<UCheatManager>,
    player_input: *const UPlayerInput,
    active_force_feedback_effects: TArray<FActiveForceFeedbackEffect>,
    // TODO: implement TMap
    dynamic_force_feedbacks: TMap<i32, FDynamicForceFeedbackDetails>,
    active_haptic_effect_left: TSharedPtr<FActiveHapticFeedbackEffect>,
    active_haptic_effect_right: TSharedPtr<FActiveHapticFeedbackEffect>,
    active_haptic_effect_gun: TSharedPtr<FActiveHapticFeedbackEffect>,
    pending_map_change_level_names: TArray<FName>,
//    b_cinematic_mode: Bool32,
//    b_hide_pawn_in_cinematic_mode: Bool32,
//    b_is_using_streaming_volumes: Bool32,
//    b_player_is_waiting: Bool32,
    bitfield2: Bool32,
    net_player_index: u8,
    mute_list: FPlayerMuteList,
    pending_swap_connection: UNetConnection,
    net_connection: UNetConnection,
    rotation_input: FRotator,
    input_yaw_scale: f32,
    input_pitch_scale: f32,
    input_roll_scale: f32,
//    b_show_mouse_cursor: Bool32,
//    b_enable_click_events: Bool32,
//    b_enable_touch_events: Bool32,
//    b_enable_mouse_over_events: Bool32,
//    b_enable_touch_over_events: Bool32,
//    b_force_feedback_enabled: Bool32,
    bitfield3: Bool32,
    force_feedback_scale: f32,
    client_event_keys: TArray<FKey>,
    default_mouse_cursor: TEnumAsByte<EMouseCursorType>,
    current_mouse_cursor: TEnumAsByte<EMouseCursorType>,
    default_click_trace_channel: TEnumAsByte<ECollisionChannel>,
    current_click_trace_channel: TEnumAsByte<ECollisionChannel>,
    hit_result_trace_distance: f32,
    force_feedback_values: FForceFeedbackValues,
    current_clickable_primitive: TWeakObjectPtr<UPrimitiveComponent>,
    current_touchable_primitaves: [TWeakObjectPtr<UPrimitiveComponent>; 11],
    current_input_stack: TArray<TWeakObjectPtr<UInputComponent>>,
    inactive_state_input_component: *const UInputComponent,
//    b_cinema_disable_input_move: Bool32,
//    b_cinema_disable_input_look: Bool32,
//    b_input_enabled: Bool32,
//    b_should_perform_full_tick_when_paused: Bool32,
    bitfield4: Bool32,
    virtual_joystick: TSharedPtr<SVirtualJoystick>,
    current_touch_interface: *const UTouchInterface,
    timer_handle_un_freeze: FTimerHandle,
    timer_handle_delayed_prepare_map_change: FTimerHandle,
    timer_handle_client_commit_map_change: FTimerHandle,
//    b_override_audio_listener: Bool32,
    bitfield5: Bool32,
    audio_listener_component: TWeakObjectPtr<USceneComponent>,
    audio_listener_location_override: FVector,
    audio_listener_rotation_override: FRotator,
    spectator_pawn: *const ASpectatorPawn,
    spawn_location: FVector,
    last_retry_player_time: f32,
    b_is_local_player_controller: bool,
    seamless_travel_count: u16,
    last_completed_seamless_travel_count: u16,
}

#[repr(C)]
pub struct FPlayerMuteList {
    voice_mute_list: TArray<TSharedRef<FUniqueNetId>>,
    gameplay_voice_mute_list: TArray<TSharedRef<FUniqueNetId>>,
    voice_packet_filter: TArray<TSharedRef<FUniqueNetId>>,
    b_has_voice_handshake_completed: bool,
    voice_channel_idx: i32,
}

lazy_static! {
    pub(in native) static ref CONTROLLER: Static<usize> = Static::new();
}

impl AController {
    pub fn rotation() -> (f32, f32, f32) {
        let pitch = unsafe { *AController::pitch_ptr() };
        let yaw = unsafe { *AController::yaw_ptr() };
        let roll = unsafe { *AController::roll_ptr() };
        (pitch, yaw, roll)
    }

    pub fn set_rotation(pitch: f32, yaw: f32, roll: f32) {
        unsafe {
            *AController::pitch_ptr() = pitch;
            *AController::yaw_ptr() = yaw;
            *AController::roll_ptr() = roll;
        }
    }
}

hook! {
    "AController::GetControlRotation",
    ACONTROLLER_GETCONTROLROTATION,
    hook,
    unhook,
    get,
    true,
}

hook_fn_once! {
    get,
    save,
    unhook,
    ACONTROLLER_GETCONTROLROTATION,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aplayercontroller_offsets() {
        unsafe {
            let ctrl: *const APlayerController = ::std::ptr::null();
            ptr_eq!(ctrl, my_hud, 0x410, "APlayerController::my_hud");
        }
    }
}
