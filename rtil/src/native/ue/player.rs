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
