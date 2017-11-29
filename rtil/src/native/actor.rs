use native::ue::*;
use native::pawn::APawn;

#[repr(C)]
pub struct AActor<B> {
    base: UObject<B>, // 0x000
    primary_actor_tick: FActorTickFunction, // 0x030
    custom_time_dilation: f32, // 0x088
    b_hidden: Bool8,
    b_net_temporary: Bool8,
    b_net_startup: Bool8,
    b_only_relevant_to_owner: Bool8,
    b_always_relevant: Bool8,
    b_replicate_movement: Bool8,
    b_tear_off: Bool8,
    b_exchanged_roles: Bool8,
    b_pending_net_update: Bool8,
    b_net_load_on_client: Bool8,
    b_net_use_owner_relevancy: Bool8,
    b_block_input: Bool8,
    b_running_user_construction_script: Bool8,
    b_allow_tick_before_begin_play: Bool8,
    b_has_finished_spawning: Bool8,
    b_tick_function_registered: Bool8,
    b_has_deferred_component_registration: Bool8,
    b_actor_enable_collision: Bool8,
    b_net_checked_initial_physics_state: Bool8,
    b_replicates: Bool8,
    remote_role: ENetRole,
    owner: *const AActor<()>,
    net_driver_name: FName,
    replicated_movement: FRepMovement,
    initial_life_span: f32,
    attachment_replication: FRepAttachment,
    role: ENetRole,
    net_dormancy: ENetDormancy,
    auto_receive_input: EAutoReceiveInputType,
    input_priority: i32,
    input_component: *const UInputComponent,
    net_cull_distance_squared: f32,
    net_tag: i32,
    net_update_time: f32,
    net_update_frequency: f32,
    min_net_update_frequency: f32,
    net_priority: f32,
    last_net_update_time: f32,
    b_auto_destroy_when_finished: Bool8,
    b_can_be_damaged: Bool8,
    b_actor_is_being_destroyed: Bool8,
    b_collide_when_placing: Bool8,
    b_find_camera_component_when_view_target: Bool8,
    b_relevant_for_network_replays: Bool8,
    b_generate_overlap_events_during_level_streaming: Bool8,
    b_can_be_in_cluster: Bool8,
    b_allow_receive_tick_event_on_dedicated_server: Bool8,
    b_actor_initialized: Bool8,
    actor_has_begun_play: EActorBeginPlayState,
    b_actor_seamless_traveled: Bool8,
    b_ignores_origin_shifting: Bool8,
    b_enable_auto_lod_generation: Bool8,
    spawn_collision_handling_method: ESpawnActorCollisionHandlingMethod,
    creation_time: f32,
    instigator: *const APawn<()>,
    children: TArray<*const AActor<()>>,
    pub root_component: *const USceneComponent, // 0x168 / 0x11c
    controlling_matinee_actors: TArray<*const AMatineeActor>,
    timer_handle_life_span_expired : FTimerHandle,
    layers: TArray<FName>,
    parent_component: TWeakObjectPtr<UChildActorComponent>,
    begin_play_call_depth: u32,
    tags: TArray<FName>,
    hidden_editor_views: u64,
    on_take_any_damage: *const (),
    on_take_point_damage: *const (),
    on_actor_begin_overlap: *const (),
    on_actor_end_overlap: *const (),
    on_begin_cursor_over: *const (),
    on_end_cursor_over: *const (),
    on_clicked: *const (),
    on_released: *const (),
    on_input_touch_begin: *const (),
    on_input_touch_end: *const (),
    on_input_touch_enter: *const (),
    on_input_touchleave: *const (),
    on_actor_hit: *const (),
    on_destroyed: *const (),
    on_end_play: *const (),
    blueprint_created_components: TArray<*const UActorComponent>,
    instance_components: TArray<*const UActorComponent>,
} // 0x380

#[repr(C)]
pub struct AInfo<B> {
    base: AActor<B>,
    sprite_component: *const UBillboardComponent,
} // 0x388

#[repr(C)]
pub enum ENetRole {
    None,
    SimulatedProxy,
    AutonomousProxy,
    Authority,
    Max,
}

#[repr(C)]
pub struct FRepMovement {
    linear_velocity: FVector,
    angular_velocity: FVector,
    location: FVector,
    rotation: FRotator,
    b_simulated_physics_sleep: Bool8,
    b_rep_physics: Bool8,
    location_quantization_level: EVectorQuantization,
    velocity_quantization_level: EVectorQuantization,
    rotation_quantization_level: EVectorQuantization,
}

#[repr(C)]
pub struct FRepAttachment {
    attach_parent: *const AActor<()>,
    location_offset: FVector_NetQuantize100,
    relative_scale_3d: FVector_NetQuantize100,
    rotation_offset: FRotator,
    attach_socket: FName,
    attach_component: USceneComponent,
}

#[repr(u8)]
pub enum EVectorQuantization {
    RoundWholeNumber,
    RoundOneDecimal,
    RoundTwoDecimals,
}

#[repr(C)]
pub enum ENetDormancy {
    Never,
    Awake,
    DormantAll,
    DormantPartial,
    Initial,
    Max,
}

#[repr(C)]
pub enum EAutoReceiveInputType {
    Disabled,
    Player0,
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
    Player7,
}

#[repr(u8)]
pub enum EActorBeginPlayState {
    HasNotBegunPlay,
    BeginningPlay,
    HasBegunPlay,
}

#[repr(u8)]
pub enum ESpawnActorCollisionHandlingMethod {
    Undefined,
    AlwaysSpawn,
    AdjustIfPossibleButAlwaysSpawn,
    AdjustIfPossibleButDontSpawnIfColliding,
    DontSpawnIfColliding,
}

#[repr(C)]
pub struct FTickFunction {
    tick_group: ETickingGroup, // 0x000
    end_tick_group: ETickingGroup, // 0x004
    actual_start_tick_group: ETickingGroup, // 0x008
    actual_end_tick_group: ETickingGroup, // 0x00c
    b_tick_even_when_paused: Bool8, // 0x010
    b_can_ever_tick: Bool8, // 0x011
    b_start_with_tick_enabled: Bool8, // 0x012
    b_allow_tick_on_dedicated_server: Bool8, // 0x013
    b_high_priority: Bool8, // 0x014
    b_run_on_any_thread: Bool8, // 0x015
    b_registered: bool, // 0x016
    b_was_interval: bool, // 0x017
    tick_state: ETickState, // 0x018
    // padding 0x003
    tick_visited_g_frame_counter: i32, // 0x01c
    tick_queued_g_frame_counter: i32, // 0x020
    task_pointer: *const (), // 0x10
    prerequisites: TArray<FTickPrerequisite>, // 0x018
    next: *const FTickFunction, // 0x028
    relative_tick_cooldown: f32, // 0x030
    last_tick_game_time_seconds: f32, // 0x034
    tick_interval: f32, // 0x038
    // padding 0x004
    tick_task_level: *const FTickTaskLevel, // 0x040
} // 0x048

#[repr(C)]
pub struct FActorTickFunction {
    base: FTickFunction, // 0x000
    target: *const AActor<()>, // 0x48
}

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

#[repr(u8)]
pub enum ETickState {
    Disabled,
    Enabled,
    CoolingDown,
}