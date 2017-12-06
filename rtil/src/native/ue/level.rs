use native::ue::*;

pub type FLevelTransformEvent = *const ();
pub type FLevelBoundsActorUpdateEvent = *const ();

#[repr(C)]
pub struct ULevel {
    base: UObject<UWorld>, // 0x000
    url: FUrl, // 0x038
    pub actors: TArray<*const AActor<()>>, // 0x0a0
    // not in memory (new?)
//    actors_for_gc: TArray<*const AActor<()>>,
    pub owning_world: *const UWorld, // 0x0b0
    model: *const UModel, // 0x0b8
    model_components: TArray<*const UModelComponent>, // 0x0c0
    // not in memory (new?)
//    actor_cluster: *const ULevelActorContainer,
    // #if WITH_EDITORONLY_DATA
//    level_script_blueprint: *const ULevelScriptBlueprint,
//    texture_streaming_resource_guids: TArray<FGuid>,
    num_texture_streaming_unbuilt_components: i32, // 0x0d0
    num_texture_streaming_dirty_resoucres: i32, // 0x0d4
    level_script_actor: *const ALevelScriptActor, // 0x0d8
    nav_list_start: *const ANavigationObjectBase, // 0x0e0
    nav_list_end: *const ANavigationObjectBase, // 0x0e8
    nav_data_chunks: TArray<*const UNavigationDataChunk>, // 0x0f0
    lightmap_total_size: f32, // 0x100
    shadowmap_total_size: f32, // 0x104
    static_navigable_geometry: TArray<FVector>, // 0x108
    // TODO: weird padding
    _pad: [u8; 0x10],
    streaming_texture_guids: TArray<FGuid>, // 0x120
    tick_task_level: *const FTickTaskLevel, // 0x130
    precomputed_light_volume: *const FPrecomputedLightVolume, // 0x138
    precomputed_volumetric_lightmap: *const FPrecomputedVolumetricLightmap, // 0x140
    precomputed_visibility_handler: FPrecomputedVisibilityHandler, // 0x148
    precomputed_volume_distance_field: FPrecomputedVolumeDistanceField, // 0x178
    remove_from_scene_fence: FRenderCommandFence, // 0x1b0
    b_is_lightning_scenario: bool, // 0x1b8
    level_build_data_id: FGuid, // 0x1bc
    pub map_build_data: *const UMapBuildDataRegistry, // 0x1d0
    light_build_level_offset: FIntVector, // 0x1d8
//    b_are_components_currently_registered: Bool8,
//    b_geometry_dirty_for_lighting: Bool8,
//    b_texture_streaming_rotation_changed: Bool8,
//    b_is_visible: Bool8,
//    b_locked: Bool8,
//    b_already_moved_actors: Bool8,
//    b_already_shifted_actors: Bool8,
//    b_already_updated_components: Bool8,
    bitfield: Bool8, // 0x1e4
//    b_already_associated_streamable_resources: Bool8,
//    b_already_initialized_network_actors: Bool8,
//    b_already_routed_actor_initialize: Bool8,
//    b_already_sorted_actor_list: Bool8,
//    b_is_associating_level: Bool8,
//    b_require_full_visibility_to_render: Bool8,
//    b_client_only_visible: Bool8,
//    b_was_duplicated_for_pie: Bool8,
    bitfield2: Bool8, // 0x1e5
//    b_is_being_removed: Bool8,
//    b_has_rerun_construction_scripts: Bool8,
//    b_actor_cluster_created: Bool8,
    bitfield3: Bool8, // 0x1e6
    current_actor_index_for_update_components: i32, // 0x1e8
    current_actor_index_for_unregister_components: i32, // 0x1e4
    // TODO: weird padding
    _pad2: [Unk8; 0xd8],
    on_apply_level_transform: FLevelTransformEvent, // 0x2c0
    // #if WITH_EDITORONLY_DATA
//    level_simplification: [FLevelSimplificationDetails; 4],
//    level_color: FLinearColor,
//    fixup_override_vertex_colors_time: f32,
//    fixup_override_vertex_colors_count: i32,
    level_bounds_actor: TWeakObjectPtr<ALevelBounds>, // 0x2c8
    instance_foliage_actor: TWeakObjectPtr<AInstanceFoliageActor>, // 0x2d0
    level_bounds_actor_updated_event: FLevelBoundsActorUpdateEvent, // 0x2d8
    pub world_settings: *const AWorldSettings, // 0x2e0
    cached_level_collection: *const FLevelCollection, // 0x2e8
    asset_user_data: TArray<*const UAssetUserData>, // 0x2f0
    pending_auto_receive_input_actors: TArray<FPendingAutoReceiveInputActor>, // 0x300
}

impl ULevel {
    pub unsafe fn get_raw() -> *const ULevel {
        (*UWorld::get_raw()).current_level
    }
}

#[repr(C)]
pub struct FPrecomputedVisibilityHandler {
    precomputed_visibility_cell_bucket_origin_xy: FVector2D, // 0x000
    precomputed_visibility_cell_size_xy: f32, // 0x008
    precomputed_visibility_cell_size_z: f32, // 0x00c
    precomputed_visibility_cell_bucket_size_xy: i32, // 0x010
    precomputed_visibility_num_cell_buckets: i32, // 0x014
    id: i32, // 0x018
    precomputed_visibility_cell_buckets: TArray<FPrecomputedVisibilityBucket>, // 0x020
} // 0x030

#[repr(C)]
pub struct FPrecomputedVolumeDistanceField {
    volume_max_distance: f32, // 0x000
    volume_box: FBox, // 0x004
    volume_size_x: i32, // 0x020
    volume_size_y: i32, // 0x024
    volume_size_z: i32, // 0x028
    data: TArray<FColor>, // 0x030
} // 0x040
