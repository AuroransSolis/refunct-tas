use native::ue::*;

#[repr(C)]
pub struct FNavLocation {
    location: FVector,
    node_ref: NavNodeRef,
}

pub type NavNodeRef = u64;

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
