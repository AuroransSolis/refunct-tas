use native::ue::*;
use native::pawn::APawn;

use native::AMYCHARACTER_TICK;
use native::ue::FVector;
#[cfg(unix)] use native::linux::character::save;
#[cfg(windows)] use native::windows::character::save;

#[repr(C)]
pub struct ACharacter<B> {
    pub base: APawn<B>,
    mesh: *const USkeletalMeshComponent,
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
    b_is_crouched: Bool32,
    b_pressed_jump: Bool32,
    b_client_updating: Bool32,
    b_client_was_falling: Bool32,
    b_client_resimulate_root_motion: Bool32,
    b_client_resimulate_root_motion_sources: Bool32,
    b_sim_gravity_disabled: Bool32,
    b_client_check_encroachment_on_net_update: Bool32,
    b_server_move_ignore_root_motion: Bool32,
    jump_key_hold_time: f32,
    jump_max_hold_time: f32,
    jump_max_count: i32,
    jump_current_count: i32,
    b_jump_max_count_exceeded: Bool32,
    b_was_jumping: Bool32,
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
pub struct FRootMotionSourceGroup {
    root_motion_sources: TArray<TSharedPtr<FRootMotionSource>>,
    pending_add_root_moution_sources: TArray<TSharedPtr<FRootMotionSource>>,
    b_has_additive_sources: bool,
    b_has_override_sources: bool,
    last_pre_additive_velocity: FVector_NetQuantize10,
    b_is_additive_velocity_applied: bool,
    last_accumulated_settings: FRootMotionSourceSettings,
}

#[repr(C)]
pub struct FRootMotionSourceSettings {
    flags: u8,
}

#[repr(C)]
pub struct FRootMotionMovementParams {
    root_motion_scale: FVector,
    b_has_root_motion: bool,
    blend_weight: f32,
    root_motion_transform: FTransform,
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

lazy_static! {
    pub(in native) static ref CHARACTER: Static<usize> = Static::new();
}

pub struct AMyCharacter {
    pub base: ACharacter<()>,
}

impl AMyCharacter {
    pub fn location() -> (f32, f32, f32) {
        let root = AMyCharacter::root_component();
        unsafe {
            ((*root).location.x, (*root).location.y, (*root).location.z)
        }
    }
    pub fn set_location(x: f32, y: f32, z: f32) {
        let root = AMyCharacter::root_component();
        unsafe {
            (*root).location = FVector { x, y, z };
        }
    }
    pub fn velocity() -> (f32, f32, f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            let FVector { x, y, z } = (*movement).velocity;
            (x, y, z)
        }
    }
    pub fn set_velocity(x: f32, y: f32, z: f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            (*movement).velocity = FVector { x, y, z };
        }
    }
    pub fn acceleration() -> (f32, f32, f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            let FVector { x, y, z } = (*movement).acceleration;
            (x, y, z)
        }
    }
    pub fn set_acceleration(x: f32, y: f32, z: f32) {
        let movement = AMyCharacter::movement();
        unsafe {
            (*movement).acceleration = FVector { x, y, z };
        }
    }
}

hook! {
    "AMyCharacter::Tick",
    AMYCHARACTER_TICK,
    hook,
    unhook,
    get,
    true,
}

hook_fn_once! {
    get,
    save,
    unhook,
    AMYCHARACTER_TICK,
}

