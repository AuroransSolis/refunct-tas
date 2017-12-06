use native::ue::*;

#[repr(C)]
pub struct FUrl {
    protocol: FString, // 0x000
    host: FString, // 0x010
    port: i32, // 0x020
    map: FString, // 0x028
    redirect_url: FString, // 0x038
    op: TArray<FString>, // 0x048
    portal: FString, // 0x058
    valid: i32, // 0x068
} // 0x06c

#[repr(C)]
pub struct FTickFunction {
    tick_group: TEnumAsByte<ETickingGroup>,
    end_tick_group: TEnumAsByte<ETickingGroup>,
    actual_start_tick_group: TEnumAsByte<ETickingGroup>,
    actual_end_tick_group: TEnumAsByte<ETickingGroup>,
    //    b_tick_even_when_paused: Bool8,
    //    b_can_ever_tick: Bool8,
    //    b_start_with_tick_enabled: Bool8,
    //    b_allow_tick_on_dedicated_server: Bool8,
    //    b_high_priority: Bool8,
    //    b_run_on_any_thread: Bool8,
    bitfield: Bool8,
    b_registered: bool,
    b_was_interval: bool,
    tick_state: ETickState,
    tick_visited_g_frame_counter: i32,
    tick_queued_g_frame_counter: i32,

    task_pointer: *const (), // 0x10
    prerequisites: TArray<FTickPrerequisite>, // 0x018
    next: *const FTickFunction, // 0x028
    relative_tick_cooldown: f32, // 0x030
    last_tick_game_time_seconds: f32, // 0x034
    tick_interval: f32, // 0x038
    tick_task_level: *const FTickTaskLevel, // 0x040
} // 0x048

#[repr(u8)]
pub enum ETickState {
    Disabled,
    Enabled,
    CoolingDown,
}

#[repr(C)]
pub struct FActorTickFunction {
    base: FTickFunction, // 0x000
    target: *const AActor<()>, // 0x48
} // 0x050

#[repr(C)]
pub enum ETickingGroup {
    PrePhysics,
    StartPhysics,
    DuringPhysics,
    EndPhysics,
    PostPhysics,
    PostUpdateWork,
    LastDemotable,
    NewlySpawned,
    Max,
}

#[repr(C)]
pub struct FActorComponentTickFunction {
    base: FTickFunction, // 0x000
    target: *const UActorComponent<()>, // 0x048
} // 0x050

#[repr(u8)]
pub enum EAutoPossessAi {
    Disabled,
    PlacedInWorld,
    Spawned,
    PlacedInWorldOrSpawned,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of() {
        use std::mem::size_of;
        assert_eq!(size_of::<FTickFunction>(), 0x48);
        assert_eq!(size_of::<FActorTickFunction>(), 0x50);
        assert_eq!(size_of::<FActorComponentTickFunction>(), 0x050);
    }
}
