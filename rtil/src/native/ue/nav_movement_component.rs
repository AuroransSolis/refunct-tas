use native::ue::*;

pub struct UNavMovementComponent {
    pub base: UMovementComponent,
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
