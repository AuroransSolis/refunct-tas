use native::ue::*;
use native::movementcomponent::UMovementComponent;

pub struct UNavMovementComponent {
    base: UMovementComponent,
    nav_agent_props: FNavAgentProperties,
    fixed_path_braking_distance: f32,
//    b_update_nav_agent_with_owners_collsion: Bool32,
//    b_use_acceleration_for_paths: Bool32,
//    b_use_fixed_braking_distance_for_path: Bool32,
//    b_stop_movement_abort_paths: Bool32,
    bitfield: Bool32,
    movement_state: FMovementProperties,
    path_following_comp: TWeakObjectPtr<UPathFollowingComponent>,
}

#[repr(C)]
pub struct FNavAgentProperties {
    base: FMovementProperties, // 0x000
    agent_radius: f32, // 0x004
    agent_height: f32, // 0x008
    agent_step_height: f32, // 0x00c
    nav_walking_search_height_scale: f32, // 0x010
    preferred_nav_data: TSubclassOf<ANavigationData>, // 0x018
} // 0x020

#[repr(C)]
pub struct FMovementProperties {
//    b_can_crouch: Bool32,
//    b_can_jump: Bool32,
//    b_can_walk: Bool32,
//    b_can_swim: Bool32,
//    b_can_fly: Bool32,
    bitfield: Bool32, // 0x000
} // 0x004
