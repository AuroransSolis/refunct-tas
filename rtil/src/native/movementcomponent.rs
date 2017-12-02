use native::ue::*;
use native::actorcomponent::UActorComponent;
use native::scenecomponent::USceneComponent;

pub struct UMovementComponent {
    base: UActorComponent<()>,
    updated_component: *const USceneComponent,
    updated_primitive: *const UPrimitiveComponent,
    move_component_flags: EMoveComponentFlags,
    pub velocity: FVector, // 0x104
//    b_contrain_to_plane: Bool32,
//    b_snap_to_plane_at_start: Bool32,
    bitfield: Bool32,
    plane_constraint_axis_setting: EPlaneConstraintAxisSetting,
    plane_constraint_normal: FVector,
    plane_constraint_origin: FVector,
//    b_update_only_if_rendered: Bool32,
//    b_auto_update_tick_registration: Bool32,
//    b_tick_before_owner: Bool32,
//    b_auto_register_updated_component: Bool32,
    bitfield2: Bool32,
    b_in_on_register: bool,
    b_in_initialize_component: bool,
}

#[repr(C)]
enum EMoveComponentFlags {
    MovecompNoFlags,
    MovecompIgnoreBases,
    MovecompSkipPhysicsMove,
    MovecompNeverIgnoreBlockingOverlaps,
    MovecompDisableBlockingOverlapDispatch,
}

#[repr(u8)]
enum EPlaneConstraintAxisSetting {
    Custom,
    X,
    Y,
    Z,
    UseGlobalPhysicsSetting,
}
