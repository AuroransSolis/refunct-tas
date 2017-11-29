use native::character::{CHARACTER, AMyCharacter};
use native::ue::FVector;

impl AMyCharacter {
    pub(in native) fn root_component() -> *mut USceneComponent {
        unsafe { *((&*CHARACTER.get() + 0x168) as *const *mut USceneComponent) }
    }
    pub(in native) fn movement() -> *mut UCharacterMovementComponent {
        unsafe { *((&*CHARACTER.get() + 0x3f0) as *const *mut UCharacterMovementComponent) }
    }
}

#[repr(C, packed)]
pub(in native) struct USceneComponent {
    _pad: [u8; 0x1a0],
    pub location: FVector,
}

#[repr(C, packed)]
pub(in native) struct UCharacterMovementComponent {
    _pad: [u8; 0x104],
    pub velocity: FVector,
    _pad2: [u8; 0x178],
    pub acceleration: FVector,
}

#[inline(never)]
pub(in native) extern "C" fn save(this: usize) {
    CHARACTER.set(this);
    log!("Got AMyCharacter: {:#x}", this);
    log!("Got AMyCharacter::RootComponent: {:#x}", AMyCharacter::root_component() as usize);
    log!("Got AMyCharacter::Movement: {:#x}", AMyCharacter::movement() as usize);
    use ::std::mem::size_of;
    use native::ue::*;
    use native::character::*;
    use native::pawn::*;
    use native::actor::*;
    log!("ACharacter: {:#x}", size_of::<ACharacter<()>>());
    log!("APawn: {:#x}", size_of::<APawn<()>>());
    log!("AActor: {:#x}", size_of::<AActor<()>>());
    log!("AInfo: {:#x}", size_of::<AActor<()>>());
    log!("FActorTickFunction: {:#x}", size_of::<FActorTickFunction>());
    log!("FTickFunction: {:#x}", size_of::<FTickFunction>());
    log!("UObject: {:#x}", size_of::<UObject<()>>());
    log!("UObjectBase: {:#x}", size_of::<UObjectBase<()>>());
    unsafe {
        let character: *const AMyCharacter = ::std::mem::transmute(this);
//        assert_eq!(&(*character).base.base.base.root_component as *const _ as isize - character as isize, 0x168);
//        assert_eq!(&(*character).base.character_movement as *const _ as isize - character as isize, 0x3c0);
    }
}
