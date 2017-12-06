use native::ue::*;

pub type FActorComponentActivateSignature = *const ();
pub type FActorComponentDeactivateSignature = *const ();

#[repr(C)]
pub struct UActorComponent<B> {
    base: UObject<B>, // 0x000
    primary_component_tick: FActorComponentTickFunction, // 0x030
    component_tags: TArray<FName>, // 0x080
    asset_user_data: TArray<*const UAssetUserData>, // 0x090
//    b_registered: Bool8,
//    b_render_state_created: Bool8,
//    b_physics_state_created: Bool8,
//    b_replicates: Bool8,
//    b_net_addressable: Bool8,
//    b_render_state_dirty: Bool8,
//    b_render_transform_dirty: Bool8,
//    b_render_dynamic_data_dirty: Bool8,
    bitfield: Bool8, // 0xa0
//    b_routed_post_rename: Bool8,
//    b_auto_register: Bool8,
//    b_allow_reregistration: Bool8,
//    b_tick_in_editor: Bool8,
//    b_never_needs_render_update: Bool8,
//    b_allow_concurrent_tick: Bool8,
//    b_allow_anyone_to_destroy_me: Bool8,
//    b_auto_activate: Bool8,
    bitfield2: Bool8, // 0xa1
//    b_is_active: Bool8,
//    b_editable_when_inherited: Bool8,
//    b_navigation_relevant: Bool8,
//    b_can_ever_affect_navigation: Bool8,
//    b_wants_initialize_component: Bool8,
//    b_wants_begin_play: Bool8,
//    b_is_editor_only: Bool8,
//    b_has_been_created: Bool8,
    bitfield3: Bool8, // 0xa2
//    b_has_been_initialized: Bool8,
//    b_has_begun_play: Bool8,
//    b_is_being_destroyed: Bool8,
//    b_tick_functions_registered: Bool8,
//    b_is_net_startup_component: Bool8,
//    marked_for_end_of_frame_update_state: (Bool8, Bool8),
    bitfield4: Bool8, // 0xa3
    creation_method: EComponentCreationMethod, // 0xa4
    // TODO: Are these really missing?
//    usc_modified_properties: TArray<FSimpleMemberReference>, // 0x098
//    on_component_activated: FActorComponentActivateSignature, // 0x0a0
    on_component_deactivated: FActorComponentDeactivateSignature, // 0x0a8
    pub owner_private: *const AActor<()>, // 0x0b0
    pub world_private: *const UWorld, // 0x0b8
} // 0x0c0

#[repr(u8)]
pub enum EComponentCreationMethod {
    Native,
    SimpleConstructionScript,
    UserConstructionScript,
    Instance,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn actorcomponent_offsets() {
        unsafe {
            let actorcomponent: *const UActorComponent<()> = ::std::ptr::null();
            ptr_eq!(actorcomponent, owner_private, 0x0b0);
            ptr_eq!(actorcomponent, world_private, 0x0b8);
        }
    }

    #[test]
    fn actorcomponent_size() {
        use std::mem::size_of;
        assert_eq!(size_of::<UActorComponent<()>>(), 0x0c0);
    }
}
