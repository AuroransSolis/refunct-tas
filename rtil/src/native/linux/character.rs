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
    unsafe {
        let character: *const AMyCharacter = ::std::mem::transmute(this);
        assert_eq!(size_of::<UObjectBase<()>>(), 0x30, "UObjectBase");
        assert_eq!(size_of::<UObject<()>>(), 0x30, "UObject");
        assert_eq!(size_of::<FTickFunction>(), 0x48, "FTickFunction");
        assert_eq!(size_of::<FActorTickFunction>(), 0x50, "FActorTickFunction");
        assert_eq!(size_of::<AActor<()>>(), 0x380, "AActor");
        assert_eq!(size_of::<AInfo<()>>(), 0x388, "AInfo");
        assert_eq!(size_of::<APawn<()>>(), 0x3e8, "APawn");
        assert_eq!(&(*character).base.base.base.custom_time_dilation as *const _ as isize - character as isize, 0x80, "Custom Time Dilation");
        assert_eq!(&(*character).base.base.base.input_component as *const _ as isize - character as isize, 0x120, "Input Component");
        assert_eq!(&(*character).base.base.base.instigator as *const _ as isize - character as isize, 0x150, "Instigator");
        assert_eq!(&(*character).base.base.base.root_component as *const _ as isize - character as isize, 0x168, "Root Component");
        assert_eq!(&(*character).base.base.base.instance_components as *const _ as usize - character as usize, 0x2c0, "Instance Components");
        assert_eq!(&(*character).base.base.controller as *const _ as usize - character as usize, 0x3c0, "Controller");
        assert_eq!(&(*character).base.character_movement as *const _ as isize - character as isize, 0x3f0, "Character Movement");
    }
}
