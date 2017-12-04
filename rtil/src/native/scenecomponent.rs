use native::ue::*;
use native::actorcomponent::UActorComponent;

type FPhysicsVolumeChanged = *const ();
type FTransformUpdated = *const ();

#[repr(C)]
pub struct USceneComponent {
    base: UActorComponent<()>, // 0x000
    cached_level_collection: *const FLevelCollection,
    physics_volume: TWeakObjectPtr<APhysicsVolume>,
    attach_parent: *const USceneComponent,
    attach_socket_name: FName,
    attach_children: TArray<*const USceneComponent>,
    client_attached_children: TArray<*const USceneComponent>,
    net_old_attach_parent: *const USceneComponent,
    bounds: FBoxSphereBounds,
    // TODO: weird padding
    _pad: [u8; 0x7c],
    pub relative_location: FVector, // 0x1a0 / 0x140
    relative_rotation: FRotator,
    relative_scale_3d: FVector,
    component_to_world: FTransform,
    component_velocity: FVector,
//    b_component_to_world_updated: Bool8,
//    b_absolute_location: Bool8,
//    b_absolute_rotation: Bool8,
//    b_absolute_scale: Bool8,
//    b_visible: Bool8,
//    b_hidden_in_game: Bool8,
//    b_should_update_physics_volume: Bool8,
//    b_bounds_change_traggers_streaming_data_rebuild: Bool8,
    bitfield: Bool8,
//    b_use_attach_parent_bound: Bool8,
//    b_disable_detachment_update_overlaps: Bool8,
//    b_wants_on_update_transform: Bool8,
//    b_net_update_transform: Bool8,
//    b_net_update_attachment: Bool8,
//    b_absolute_translation_deprecated: Bool8,
    bitfield2: Bool8,
    mobility: TEnumAsByte<EComponentMobilityType>,
    detail_mode: TEnumAsByte<EDetailMode>,
    world_rotation_cache: FRotationConversionCache,
    relative_rotation_cache: FRotationConversionCache,
    physics_volume_changed_delete: FPhysicsVolumeChanged,
    scoped_movement_stack: TArray<*const FScopedMovementUpdate>,
    transform_updated: FTransformUpdated,
}

#[repr(C)]
pub enum EComponentMobilityType {
    Static,
    Stationary,
    Movable,
}

#[repr(u8)]
pub enum EVisibilityPropagation {
    NoPropagation,
    DirtyOnly,
    Propagate,
}

#[repr(C)]
pub enum EDetailMode {
    DmLow,
    DmMedium,
    DmHigh,
    DmMax,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uscenecomponent_offsets() {
        unsafe {
            let scenecomponent: *const USceneComponent = ::std::ptr::null();
            ptr_eq!(scenecomponent, relative_location, 0x1a0);
        }
    }
}