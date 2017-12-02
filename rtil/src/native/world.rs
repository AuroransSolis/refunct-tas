use native::ue::*;
use native::AController;
use native::gameinstance::UMyGameInstance;
use native::gamestate::AGameState;
use native::pawn::APawn;
use native::level::ULevel;
use native::actorcomponent::UActorComponent;
use native;

#[repr(C)]
pub struct UWorld {
    base: UObject<UPackage>,
    persistent_level: *const ULevel, // 0x030 /
    net_driver: *const UNetDriver, // 0x038
    line_batcher: *const ULineBatchComponent, // 0x040
    persistent_line_batcher: *const ULineBatchComponent, // 0x048
    foreground_line_batcher: *const ULineBatchComponent, // 0x050
    network_manager: *const AGameNetworkManager, // 0x058
    physics_collision_handler: *const UPhysicsCollisionHandler, // 0x060
    extra_referenced_objects: TArray<*const UObject<()>>, // 0x068
    per_module_data_objects: TArray<*const UObject<()>>, // 0x078
    pub streaming_levels: TArray<*const ULevelStreaming>, // 0x088
    streaming_levels_prefix: *const FString, // 0x098
    current_level_pending_visibility: *const ULevel, // 0x0a0
    current_level_pending_invisibility: *const ULevel, // 0x0a8
    demo_net_driver: *const UDemoNetDriver, // 0x0b0
    my_particle_event_manager: *const AParticleEventManager, // 0x0b8
    default_physics_volume: *const APhysicsVolume, // 0x0c0
    view_locations_rendered_list_frame: TArray<FVector>, // 0x0c8
//    b_world_was_loaded_this_tick: Bool32, // 0x0d8
//    b_trigger_post_load_map: Bool32, // 0x0dc
    bitfield: Bool32,
    navigation_system: *const UNavigationSystem, // 0x0e0
    authority_game_mode: *const AGameMode, // 0x0e8
    game_state: *const AGameState, // 0x0f0
    ai_system: *const UAISystemBase, // 0x0f8
    avoidance_manager: *const UAvoidanceManager, // 0x100
    pub levels: TArray<*const ULevel>, // 0x108
    level_collections: TArray<FLevelCollection>, // 0x118
    // for some reason this is instead some sort of pointer in-memory of the game
    //active_level_collection_index: i32,
    _unk3: usize, // 0x128
    pub current_level: *const ULevel, // 0x130
    owning_game_instance: *const UMyGameInstance, // 0x138
    parameter_collection_instances: TArray<*const UMaterialParameterCollectionInstance>, // 0x140
    canvas_for_rendering_to_target: *const UCanvas, // 0x150
    canvas_for_draw_material_to_render_target: *const UCanvas, // 0x158
    scene: *const FScene, // 0x160
    feature_level: ERHIFeatureLevelType, // 0x168
    // padding 0x04
    controller_list: TArray<TWeakObjectPtr<AController>>, // 0x170
    player_controller_list: TArray<TWeakObjectPtr<APlayerController>>, // 0x180
    pawn_list: TArray<TWeakObjectPtr<APawn<()>>>, // 0x190
    auto_camera_actor_list: TArray<TWeakObjectPtr<ACameraActor>>, // 0x1a0
    non_default_physics_volume_list: TArray<APhysicsVolume>, // 0x1b0
    physics_scene: *const FPhysScene, // 0x1c0
    // for now unsized, so rip
    components_that_need_end_of_frame_update: TSet<TWeakObjectPtr<UActorComponent<()>>>,
    components_that_need_end_of_frame_update_on_game_thread: TSet<TWeakObjectPtr<UActorComponent<()>>>,
    // TODO: padding for unsized types
    async_trace_state: FWorldAsyncTraceState,
    // ... World.h:988 following
}

impl UWorld {
    pub unsafe fn get_raw() -> *const UWorld {
        native::GWORLD as *const UWorld
    }
}

#[repr(C)]
struct FWorldAsyncTraceState {
    data_buffer: [AsyncTraceData; 2],
    current_frame: i32,
}

type FGraphEventArray = TArray<FGraphEventRef>;

#[repr(C)]
struct AsyncTraceData {
    trace_data: TArray<TUniquePtr<TTraceThreadData<FTraceDatum>>>,
    overlap_data: TArray<TUniquePtr<TTraceThreadData<FOverlapDatum>>>,
    num_queued_trace_data: i32,
    num_queued_overlap_data: i32,
    b_async_allowed: bool,
    async_trace_completion_event: FGraphEventArray,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uworld_offsets() {
        unsafe {
            let world: *const UWorld = ::std::ptr::null();
            ptr_eq!(world, streaming_levels, 0x088, "UWorld.streaming_levels");
            ptr_eq!(world, levels, 0x108, "UWorld.levels");
            ptr_eq!(world, current_level, 0x130, "UWorld.current_level");
        }
    }
}
