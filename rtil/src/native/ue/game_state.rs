use native::ue::*;

#[repr(C)]
pub struct AGameState {
    base: AGameStateBase<ULevel>, // 0x000
    match_state: FName, // 0x3c8
    previous_match_state: FName, // 0x3d0
    elapsed_time: i32, // 0x3d8
    timer_handle_default_timer: FTimerHandle, // 0x3e0
} // apparently 0x570
