use native::ue::*;
use native::actor::AInfo;

#[repr(C)]
pub struct AGameStateBase<B> {
    base: AInfo<B>, // 0x000
    game_mode_class: TSubclassOf<AGameModeBase>, // 0x388
    authority_game_mode: TSubclassOf<AGameModeBase>, // 0x390
    spectator_class: *const ASpectatorPawn, // 0x398
    player_array: TArray<*const APlayerState>, // 0x3a0
    b_replicated_has_begun_play: bool, // 0x3b0
    replicated_world_time_seconds: f32, // 0x3b4
    server_world_time_seconds_delta: f32, // 0x3b8
    server_world_time_seconds_update_frequency: f32, // 0x3bc
    timer_handle_update_server_time_seconds: FTimerHandle, // 0x3c0
} // 0x3c8

#[repr(C)]
pub struct AGameState {
    base: AGameStateBase<ULevel>, // 0x000
    match_state: FName, // 0x3c8
    previous_match_state: FName, // 0x3d0
    elapsed_time: i32, // 0x3d8
    timer_handle_default_timer: FTimerHandle, // 0x3e0
} // apparently 0x570
