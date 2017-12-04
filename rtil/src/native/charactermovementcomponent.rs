use native::ue::*;
use native::pawnmovementcomponent::UPawnMovementComponent;
use native::character::ACharacter;
use native::scenecomponent::USceneComponent;
use native::character::FRootMotionSourceGroup;
use native::actor::FTickFunction;

pub struct UCharacterMovementComponent {
    pub base: UPawnMovementComponent,
    character_owner: *const ACharacter<()>,
//    b_apply_gravity_while_jumping: Bool32,
    bitfield: Bool32,
    gravity_scale: f32,
    max_step_height: f32,
    jump_z_velocity: f32,
    jump_off_jump_z_factor: f32,
    walkable_floor_angle: f32,
    walkable_floor_z: f32,
    movement_mode: TEnumAsByte<EMovementMode>,
    custom_movement_mode: u8,
    old_base_location: FVector,
    old_base_quat: FQuat,
    ground_friction: f32,
    max_walk_speed: f32,
    max_walk_speed_crouched: f32,
    max_swim_speed: f32,
    max_fly_speed: f32,
    max_custom_movement_speed: f32,
    max_acceleration: f32,
    min_analog_walk_speed: f32,
    braking_friction_factor: f32,
    braking_friction: f32,
//    b_use_separate_braking_friction: Bool32,
    bitfield2: Bool32,
    braking_deceleration_walking: f32,
    braking_deceleration_falling: f32,
    braking_deceleration_swimming: f32,
    braking_deceleration_flying: f32,
    air_control: f32,
    air_control_boost_multiplier: f32,
    air_control_boost_velocity_threshold: f32,
    falling_lateral_friction: f32,
    crouch_half_height: f32,
    buoyancy: f32,
    perch_radius_threshold: f32,
    perch_additional_height: f32,
    rotation_rate: FRotator,
//    b_use_controller_desired_rotation: Bool32,
//    b_orient_rotation_to_movement: Bool32,
//    b_sweep_while_nav_walking: Bool32,
//    b_need_sweep_while_walking_update: Bool32,
//    b_movement_in_progress: Bool32,
//    b_enable_scoped_movement_updates: Bool32,
//    b_force_max_accel: Bool32,
//    b_run_physics_with_no_controller: Bool32,
//    b_force_next_floor_check: Bool32,
//    b_shrink_proxy_capsule: Bool32,
//    b_can_walk_off_ledges: Bool32,
//    b_can_walk_off_legdes_when_crouching: Bool32,
//    b_network_smoothing_complete: Bool32,
//    b_defer_update_move_component: Bool32,
    bitfield3: Bool32,
    deferred_update_move_component: *const USceneComponent,
    max_out_of_water_step_height: f32,
    out_of_water_z: f32,
    mass: f32,
    b_enable_physics_interaction: bool,
    b_touch_force_scaled_to_mass: bool,
    b_push_force_scaled_to_mass: bool,
    b_push_force_using_z_offset: bool,
    b_scale_push_force_to_velocity: bool,
    standing_downward_force_scale: f32,
    initial_push_force_factor: f32,
    push_force_factor: f32,
    push_force_point_z_offset_factor: f32,
    touch_force_factor: f32,
    min_touch_force: f32,
    max_touch_force: f32,
    repulsion_force: f32,
//    b_force_braking_deprecated: Bool32,
    bitfield4: Bool32,
    crouch_speed_multiplier_deprecated: f32,
    upper_impact_normal_scale_deprecated: f32,
    pub acceleration: FVector, // 0x178
    last_update_location: FVector,
    last_update_rotation: FQuat,
    last_update_velocity: FVector,
    server_last_transform_update_time_stamp: f32,
    pending_impulse_to_apply: FVector,
    pending_force_to_apply: FVector,
    analog_input_modifier: f32,
    last_stuck_warning_time: f32,
    stuck_warning_count_since_notify: f32,
    max_simulation_time_step: f32,
    max_simulation_iterations: f32,
    max_depenetration_with_geometry: f32,
    max_depenetration_with_geometry_as_proxy: f32,
    max_depenetration_with_pawn: f32,
    max_depenetration_with_pawn_as_proxy: f32,
    network_simulated_smooth_location_time: f32,
    network_simulated_smooth_rotation_time: f32,
    listen_server_network_simulated_smooth_location_time: f32,
    listen_server_network_simulated_smooth_rotation_time: f32,
    net_proxy_shrink_radius: f32,
    net_proxy_shrink_half_height: f32,
    network_max_smooth_update_distance: f32,
    network_no_smooth_update_distance: f32,
    network_smoothing_mode: ENetworkSmoothingMode,
    ledge_check_threshold: f32,
    jump_out_of_water_pitch: f32,
    current_floor: FFindFloorResult,
    default_land_movement_mode: TEnumAsByte<EMovementMode>,
    default_water_movement_mode: TEnumAsByte<EMovementMode>,
    ground_movement_mode: TEnumAsByte<EMovementMode>,
//    b_maintain_horizontal_ground_velocity: Bool32,
//    b_impart_base_velocity_x: Bool32,
//    b_impart_base_velocity_y: Bool32,
//    b_impart_base_velocity_z: Bool32,
//    b_impart_base_angular_velocity: Bool32,
//    b_just_teleported: Bool32,
//    b_network_update_received: Bool32,
//    b_network_movement_mode_changed: Bool32,
//    b_ignore_client_movement_error_checks_and_correction: Bool32,
//    b_notify_apex: Bool32,
//    b_cheat_flying: Bool32,
//    b_wants_to_crouch: Bool32,
//    b_crouch_maintains_base_location: Bool32,
//    b_ignore_base_rotation: Bool32,
//    b_fast_attached_move: Bool32,
//    b_always_check_floor: Bool32,
//    b_use_flat_base_for_floor_checks: Bool32,
//    b_performing_jump_off: Bool32,
//    b_wants_to_leave_nav_walking: Bool32,
//    b_use_rvo_avoidance: Bool32,
//    b_requested_move_use_acceleration: Bool32,
//    b_is_nav_walking_on_server: Bool32,
//    b_has_requested_velocity: Bool32,
//    b_requested_move_with_max_speed: Bool32,
//    b_was_avoidance_updated: Bool32,
//    b_use_rvo_post_process: Bool32,
//    b_defer_update_based_movement: Bool32,
//    b_project_nav_mesh_walking: Bool32,
//    b_project_nav_mesh_on_both_world_channels: Bool32,
    bitfield5: Bool32,
    avoidance_lock_velocity: FVector,
    avoidance_lock_timer: f32,
    avoidance_consideration_radius: f32,
    requested_velocity: FVector,
    avoidance_uid: i32,
    avoidance_group: FNavAvoidanceMask,
    groups_to_avoid: FNavAvoidanceMask,
    groups_to_ignore: FNavAvoidanceMask,
    avoidance_weight: f32,
    pending_launch_velocity: FVector,
    cached_nav_location: FNavLocation,
    cached_projected_nav_mesh_hit_result: FHitResult,
    nav_mesh_projection_interval: f32,
    nav_mesh_projection_timer: f32,
    nav_mesh_projection_interp_speed: f32,
    nav_mesh_projection_height_scale_up: f32,
    nav_mesh_projection_height_scale_down: f32,
    nav_walking_floor_dist_tolerance: f32,
    post_physics_tick_function: FCharacterMovementComponentPostPhysicsTickFunction,
    client_prediction_data: *const FNetworkPredictionData_Client_Character,
    server_prediction_data: *const FNetworkPredictionData_Server_Character,
    min_time_between_time_stamp_resets: f32,
    current_root_motion: FRootMotionSourceGroup,
    // TArray<_, TInlineAllocator<(uint32)ERootMotionMapping::MapSize> > (mapsize = 16)
    root_motion_id_mappings: TArray<FRootMotionServerToLocalIdMapping>,
    root_motion_params: FRootMotionMovementParams,
    anim_root_motion_velocity: FVector,
    b_was_simulation_root_motion: bool,
//    b_allow_physics_rotation_during_anim_root_motion: Bool32,
    bitfield6: Bool32,
}

impl UCharacterMovementComponent {
    pub fn velocity(&self) -> FVector {
        self.base.base.base.velocity
    }

    pub fn set_velocity(&mut self, velocity: FVector) {
        self.base.base.base.velocity = velocity;
    }

    pub fn acceleration(&self) -> FVector {
        self.acceleration
    }

    pub fn set_acceleration(&mut self, acceleration: FVector) {
        self.acceleration = acceleration
    }
}

#[repr(u8)]
pub enum ENetworkSmoothingMode {
    Disabled,
    Linear,
    Exponential,
    Replay,
}

#[repr(C)]
pub enum EMovementMode {
    None,
    Walking,
    NavWalking,
    Falling,
    Swimming,
    Flying,
    Custom,
    MAX,
}

#[repr(C)]
pub struct FFindFloorResult {
//    b_blocking_hit: Bool32,
//    b_walkable_floor: Bool32,
//    b_line_tracee: Bool32,
    bitfield: Bool32,
    floor_dist: f32,
    line_dist: f32,
    hit_result: FHitResult,
}

#[repr(C)]
pub struct FNavAvoidanceMask {
    // b_group{0..32}: Bool32,
    bitfield: Bool32
}

#[repr(C)]
pub struct FNavLocation {
    location: FVector,
    node_ref: NavNodeRef,
}

type NavNodeRef = u64;

#[repr(C)]
pub struct FHitResult {
    normal: FVector4,
    time: f32,
    item: i32,
}

#[repr(C)]
pub struct FCharacterMovementComponentPostPhysicsTickFunction {
    base: FTickFunction,
    target: *const UCharacterMovementComponent,
}

#[repr(C)]
pub struct FNetworkPredictionData_Client_Character {
    base: FNetworkPredictionData_Client,
    client_update_time: f32,
    current_time_stamp: f32,
    saved_moves: TArray<FSavedMovePtr>,
    free_moves: TArray<FSavedMovePtr>,
    pending_pove: FSavedMovePtr,
    last_acked_move: FSavedMovePtr,
    max_free_move_count: i32,
    max_saved_move_count: i32,
    root_motion_movement: FRootMotionMovementParams,
//    b_update_position: Bool32,
//    b_smooth_net_updates: Bool32,
    bitfield: Bool32,
    original_mesh_translation_offset: FVector,
    mesh_translation_offset: FVector,
    original_mesh_rotation_offset: FQuat,
    mesh_rotation_offset: FQuat,
    mesh_rotation_target: FQuat,
    last_correction_delta: f32,
    last_correction_time: f32,
    smoothing_server_time_stamp: f64,
    smoothing_client_time_stamp: f64,
    current_smooth_tume: f32,
    b_use_linear_smoothing: bool,
    max_smooth_net_update_dist: f32,
    no_smooth_net_update_dist: f32,
    smooth_net_update_time: f32,
    smooth_net_update_rotation_time: f32,
    max_response_time: f32,
    max_move_delta_time: f32,
    last_smooth_location: FVector,
    last_server_location: FVector,
    simulated_debug_draw_time: f32,
    replay_samples: TArray<FCharacterReplaySample>,
}

#[repr(C)]
pub struct FNetworkPredictionData_Server_Character {
    base: FNetworkPredictionData_Server,
    pending_adjustment: FClientAdjustment,
    current_client_time_stamp: f32,
    last_update_time: f32,
    server_time_stamp_last_server_move: f32,
    max_response_time: f32,
    max_move_delta_time: f32,
//    b_force_client_update: Bool32,
    bitfield: Bool32,
    lifetime_raw_time_discrepancy: f32,
    time_discrepancy: f32,
    b_resolving_time_discrepancy: bool,
    time_discrepancy_resolution_move_delta_override: f32,
    time_discrepancy_accumulated_client_deltas_since_last_server_tick: f32,
    world_creation_time: f32,
}

type FSavedMovePtr = TSharedPtr<FSavedMove_Character>;

#[repr(C)]
pub struct FRootMotionServerToLocalIdMapping {
    server_id: u16,
    local_id: u16,
    time_stamp: f32,
}

#[repr(C)]
pub struct FRootMotionMovementParams {
    b_has_root_motion: bool,
    blend_weight: bool,
    root_motion_transform: FTransform,
}

#[repr(C)]
pub struct FNetworkPredictionData_Client;

#[repr(C)]
pub struct FNetworkPredictionData_Server {
    server_time_stamp: f32
}

#[repr(C)]
pub struct FClientAdjustment {
    time_stamp: f32,
    delta_time: f32,
    new_loc: FVector,
    new_vel: FVector,
    new_rot: FRotator,
    new_base: *const UPrimitiveComponent,
    new_base_bone_name: FName,
    b_ack_good_move: bool,
    b_base_relative_position: bool,
    movement_mode: u8,
}
