use std::cell::RefCell;

use statics::{StaticGuard, StaticUntyped};
use native::ue::*;
use native::AMYCHARACTER_TICK;

#[repr(C)]
pub struct ACharacter<B> {
    pub base: APawn<B>,
    mesh: *const USkeletalMeshComponent, // 0x3e8
    // if WITH_EDITORONLY_DATA
    // arrow_component: *const UArrowComponent,
    pub character_movement: *const UCharacterMovementComponent, // 0x3f0 / 0x2fc
    capsule_component: *const UCapsuleComponent,
    based_movement: FBasedMovementInfo,
    replicated_based_movement: FBasedMovementInfo,
    anim_root_motion_translation_scale: f32,
    base_translation_offset: FVector,
    base_rotation_offset: FQuat,
    replicated_server_last_transform_update_time_stamp: f32,
    replicated_movement_mode: u8,
    b_in_base_replication: bool,
    crouched_eye_height: f32,
//    b_is_crouched: Bool32,
//    b_pressed_jump: Bool32,
//    b_client_updating: Bool32,
//    b_client_was_falling: Bool32,
//    b_client_resimulate_root_motion: Bool32,
//    b_client_resimulate_root_motion_sources: Bool32,
//    b_sim_gravity_disabled: Bool32,
//    b_client_check_encroachment_on_net_update: Bool32,
//    b_server_move_ignore_root_motion: Bool32,
    bitfield: Bool32,
    jump_key_hold_time: f32,
    jump_max_hold_time: f32,
    jump_max_count: i32,
    jump_current_count: i32,
//    b_jump_max_count_exceeded: Bool32,
//    b_was_jumping: Bool32,
    bitfield2: Bool32,
    on_reached_jump_apex: FCharacterReachedApexSignature,
    landed_delegate: FLandedSignature,
    movement_mode_changed_delegate: FMovementModeChangedSignature,
    on_character_movement_updated: FCharacterMovementUpdatedSignature,
    saved_root_motion: FRootMotionSourceGroup,
    client_root_motion_params: FRootMotionMovementParams,
    root_motion_rep_moves: TArray<FSimulatedRootMotionReplicatedMove>,
    rep_root_motion: FRepRootMotionMontage,
}

// Delegates
type FCharacterReachedApexSignature = *const ();
type FLandedSignature = *const ();
type FMovementModeChangedSignature = *const ();
type FCharacterMovementUpdatedSignature = *const ();

#[repr(C)]
pub struct FBasedMovementInfo {
    movement_base: * const UPrimitiveComponent,
    bone_name: FName,
    location: FVector_NetQuantize100,
    rotation: FRotator,
    b_server_has_base_component: bool,
    b_relative_rotation: bool,
    b_server_has_velocity: bool,
    // END
}

#[repr(C)]
pub struct FRepRootMotionMontage {
    b_is_active: bool,
    anim_montage: *const UAnimMontage,
    position: f32,
    location: FVector_NetQuantize100,
    rotation: FRotator,
    movement_pase: *const UPrimitiveComponent,
    movement_base_bone_name: FName,
    b_relative_position: bool,
    b_relative_rotation: bool,
    authoritative_root_motion: FRootMotionSourceGroup,
    acceleration: FVector_NetQuantize10,
    linear_velocity: FVector_NetQuantize10,
}

static CHARACTER: StaticUntyped = StaticUntyped::new();

pub struct AMyCharacter {
    pub base: ACharacter<()>,
}

impl AMyCharacter {
    pub fn get() -> StaticGuard<'static, &'static mut AMyCharacter> {
        unsafe {
            let actors = &(*ULevel::get_raw()).actors;
            let actor = actors[actors.len() - 1];
            let character = actor as *const AMyCharacter as *mut AMyCharacter;
            let character: &'static mut _ = ::std::mem::transmute(character);
            CHARACTER.lock(character)
        }
    }

    fn root_component(&self) -> &USceneComponent {
        unsafe {
            &*self.base.base.base.root_component
        }
    }

    fn root_component_mut(&mut self) -> &mut USceneComponent {
        unsafe {
            &mut *(self.base.base.base.root_component as *mut _)
        }
    }

    fn character_movement(&self) -> &UCharacterMovementComponent {
        unsafe {
            &*self.base.character_movement
        }
    }

    fn character_movement_mut(&mut self) -> &mut UCharacterMovementComponent {
        unsafe {
            &mut *(self.base.character_movement as *mut _)
        }
    }

    pub fn location(&self) -> FVector {
        self.root_component().relative_location
    }
    pub fn set_location(&mut self, location: FVector) {
        self.root_component_mut().relative_location = location;
    }

    pub fn velocity(&self) -> FVector {
        self.character_movement().velocity()
    }
    pub fn set_velocity(&mut self, velocity: FVector) {
        self.character_movement_mut().set_velocity(velocity)
    }

    pub fn acceleration(&self) -> FVector {
        self.character_movement().acceleration()
    }
    pub fn set_acceleration(&mut self, acceleration: FVector) {
        self.character_movement_mut().set_acceleration(acceleration)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_offsets() {
        use std::mem::size_of;
        assert_eq!(size_of::<UObjectBase<()>>(), 0x30, "UObjectBase");
        assert_eq!(size_of::<UObject<()>>(), 0x30, "UObject");
        assert_eq!(size_of::<APawn<()>>(), 0x3e8, "APawn");
        unsafe {
            let character: *const AMyCharacter = ::std::ptr::null();
            ptr_eq!(character, base.base.base.custom_time_dilation, 0x80, "Custom Time Dilation");
            ptr_eq!(character, base.base.base.input_component, 0x120, "Input Component");
            ptr_eq!(character, base.base.base.instigator, 0x150, "Instigator");
            ptr_eq!(character, base.base.base.root_component, 0x168, "Root Component");
            ptr_eq!(character, base.base.base.instance_components, 0x2c0, "Instance Components");
            ptr_eq!(character, base.base.controller, 0x3c0, "Controller");
            ptr_eq!(character, base.character_movement, 0x3f0, "Character Movement");
        }
    }
}
