use native::ue::*;

#[repr(C)]
pub struct FPlayerMuteList {
    voice_mute_list: TArray<TSharedRef<FUniqueNetId>>,
    gameplay_voice_mute_list: TArray<TSharedRef<FUniqueNetId>>,
    voice_packet_filter: TArray<TSharedRef<FUniqueNetId>>,
    b_has_voice_handshake_completed: bool,
    voice_channel_idx: i32,
}

