#![allow(unused)]

#[cfg(test)]
macro_rules! ptr_eq {
    ($var:ident, $($field:ident).*, $cmp:expr) => {
        assert_eq!(&(*$var).$($field).* as *const _ as usize - $var as usize, $cmp);
    };
    ($var:ident, $($field:ident).*, $cmp:expr, $args:tt) => {
        assert_eq!(&(*$var).$($field).* as *const _ as usize - $var as usize, $cmp, $args);
    }
}

mod memory;
pub use self::memory::*;
mod collections;
pub use self::collections::*;
mod math;
pub use self::math::*;
mod input_interface;
pub use self::input_interface::*;
mod primitives;
pub use self::primitives::*;
mod object;
pub use self::object::*;
mod rhi_definitions;
pub use self::rhi_definitions::*;
mod pointers;
pub use self::pointers::*;
mod stubs;
pub use self::stubs::*;
mod enum_as_byte;
pub use self::enum_as_byte::*;
mod base_types;
pub use self::base_types::*;
mod engine_types;
pub use self::engine_types::*;
mod guid;
pub use self::guid::*;
mod scene_types;
pub use self::scene_types::*;
mod actor;
pub use self::actor::*;
mod info;
pub use self::info::*;
mod actor_component;
pub use self::actor_component::*;
mod root_motion_source;
pub use self::root_motion_source::*;
mod animation_asset;
pub use self::animation_asset::*;
mod character;
pub use self::character::*;
mod navigation_avoidance_types;
pub use self::navigation_avoidance_types::*;
mod navigation_types;
pub use self::navigation_types::*;
mod network_prediction_interface;
pub use self::network_prediction_interface::*;
mod controller;
pub use self::controller::*;
mod character_controller;
pub use self::character_controller::*;
mod player_mute_list;
pub use self::player_mute_list::*;
mod game_instance;
pub use self::game_instance::*;
mod character_movement_component;
pub use self::character_movement_component::*;
mod game_state_base;
pub use self::game_state_base::*;
mod game_state;
pub use self::game_state::*;
mod render_command_fence;
pub use self::render_command_fence::*;
mod level;
pub use self::level::*;
mod movement_component;
pub use self::movement_component::*;
mod nav_movement_component;
pub use self::nav_movement_component::*;
mod pawn_movement_component;
pub use self::pawn_movement_component::*;
mod pawn;
pub use self::pawn::*;
mod local_player;
pub use self::local_player::*;
mod player;
pub use self::player::*;
mod scene_component;
pub use self::scene_component::*;
mod task_graph_interfaces;
pub use self::task_graph_interfaces::*;
mod world_collision;
pub use self::world_collision::*;
mod world;
pub use self::world::*;
