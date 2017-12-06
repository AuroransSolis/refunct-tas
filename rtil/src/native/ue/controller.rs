use native::ue::*;

pub type FInstigatedAnyDamageSignature = *const ();

#[repr(C)]
pub struct AController {
    base: AActor<ULevel>, // 0x000
    _unk: Unk8, // 0x380
    _unk2: *const (), // 0x388
    pawn: *const APawn<()>, // 0x390
    old_pawn: TWeakObjectPtr<APawn<()>>, // 0x398
    character: *const ACharacter<()>, // 0x3a0
    player_state: *const APlayerState, // 0x3a8
    transform_component: *const USceneComponent, // 0x3b0
    control_rotation: FRotator, // 0x3b8
    //    b_attach_to_pawn: Bool32,
    //    b_is_player_controller: Bool32,
    bitfield: Bool32, // 0x3c4
    // not in memory (new?)
    //    ignore_move_input: Bool8,
    //    ignore_look_input: Bool8,
    start_spot: TWeakObjectPtr<AActor<()>>, // 0x3c8
    state_name: FName, // 0x3d0
    on_instigated_any_damage: FInstigatedAnyDamageSignature, // 0x3d8
} // 0x3e0
