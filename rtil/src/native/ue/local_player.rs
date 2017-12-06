use native::ue::*;

#[repr(C)]
pub struct ULocalPlayer {
    player: UPlayer, // 0x000
    cached_unique_net_id: TSharedPtr<FUniqueNetId>, // 0x048
    viewport_client: *const UGameViewportClient, // 0x058
    origin: FVector2D, // 0x060
    size: FVector2D, // 0x068
    last_view_location: FVector, // 0x070
    // aspect_ratio_axis_contraint
    pending_level_player_controller_class: *const UClass, // 0x090
    //    b_sent_split_join: Bool32, // 0x098
    bitfield: Bool32,
    view_state: FSceneViewStateReference, // 0x09c
    stereo_view_state: FSceneViewStateReference,
    mono_view_state: FSceneViewStateReference,
    controller_id: i32,
    // other stuff
}
