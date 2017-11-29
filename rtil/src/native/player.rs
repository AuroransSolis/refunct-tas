use native::ue::*;

#[repr(C)]
pub struct UPlayer {
    vtable: *const (), // 0x000
    _unk1: [Unk4; 2], // 0x008
    class: *const UClass, // 0x010
    _unk2: [Unk4; 2], // 0x018
    engine: *const UGameEngine, // 0x020
    vtable2: *const (), // 0x028
    player_controller: *const APlayerController, // 0x030
    current_net_speed: i32, // 0x038
    configured_internet_speed: i32,// 0x03c
    configured_lan_speed: i32, // 0x040
}

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
    b_sent_split_join: Bool32, // 0x098
    view_state: FSceneViewStateReference, // 0x09c
    stereo_view_state: FSceneViewStateReference,
    mono_view_state: FSceneViewStateReference,
    controller_id: i32,
    // other stuff
}