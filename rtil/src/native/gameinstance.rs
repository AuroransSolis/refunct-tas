use native::ue::*;
use native::player::ULocalPlayer;
use native::actor::AActor;

#[repr(C, packed)]
pub struct UMyGameInstance {
    base: UObject<UGameEngine>,
    // WorldContext?
    _unk3: *const (), // 0x030
    local_players: TArray<*const ULocalPlayer>, // 0x038
    online_session: *const UOnlineSession, // 0x048
    // Currently unsized types
    notify_pre_client_travel_delegates: FOnPreClientTravel,
    on_play_together_event_receive_delegate_handle: FDelegateHandle,
    // for now padding for unsized types
    _pad: [u8; 0x80], // 0x050
    pie_map_name: *const FString, // 0x0d0
    timer_manager: *const FTimerManager, // 0x0d8
    // latent_action_manager: somewhere
    // END OF UGameInstance
    _unk4: *const (), // 0x0e0
    _unk5: Unk8, // 0x0e8
    _unk6: *const (), // 0x0f0
    save_game: *const USaveGame, // 0x0f8
    _unk7: TArray<Unk4>, // 0x100
    actors: TArray<*const AActor<()>>, // 0x110
    some_actor: *const AActor<()>, // 0x120
    _unk8: [Unk4; 0x8], // 0x128
    _unk9: [u8; 0x8], // 0x148
    _unk10: [u8; 0x30], // 0x150
    widget1: *const UUserWidget, // 0x180
    widget2: *const UUserWidget, // 0x188
    widget3: *const UUserWidget, // 0x190
    widget4: *const UUserWidget, // 0x198
    widget5: *const UUserWidget, // 0x1a0
    _unk11: [u8; 0x18], // 0x1a8
    // END
}