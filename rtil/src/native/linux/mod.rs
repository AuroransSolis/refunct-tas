#[macro_use] mod macros;
pub(in native) mod slateapp;
pub(in native) mod controller;
pub(in native) mod character;
pub(in native) mod app;

use std::env;
use std::collections::HashMap;

use libc::{self, c_void, PROT_READ, PROT_WRITE, PROT_EXEC};
use dynsym;

// Shoutout to https://github.com/geofft/redhook/blob/master/src/ld_preload.rs#L18
// Rust doesn't directly expose __attribute__((constructor)), but this
// is how GNU implements it.
#[link_section=".init_array"]
pub static INITIALIZE_CTOR: extern "C" fn() = ::initialize;

pub(in native) static mut AMYCHARACTER_FORCEDUNCROUCH: usize = 0;
pub(in native) static mut FSLATEAPPLICATION_TICK: usize = 0;
pub(in native) static mut FSLATEAPPLICATION_ONKEYDOWN: usize = 0;
pub(in native) static mut FSLATEAPPLICATION_ONKEYUP: usize = 0;
pub(in native) static mut FSLATEAPPLICATION_ONRAWMOUSEMOVE: usize = 0;
pub(in native) static mut ACONTROLLER_GETCONTROLROTATION: usize = 0;
pub(in native) static mut UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE: usize = 0;
pub(in native) static mut AMYCHARACTER_TICK: usize = 0;
pub(in native) static mut FAPP_DELTATIME: usize = 0;
pub(in native) static mut FMEMORY_MALLOC: usize = 0;
pub(in native) static mut FMEMORY_FREE: usize = 0;
pub(in native) static mut GWORLD: usize = 0x50663b8;
pub(in native) static mut UGAMEINSTANCE_STARTRECORDINGREPLAY: usize = 0;
pub(in native) static mut UGAMEINSTANCE_STOPRECORDINGREPLAY: usize = 0;
pub(in native) static mut UGAMEINSTANCE_PLAYREPLAY: usize = 0;
pub(in native) static mut UGAMEPLAYSTATICS_OPENLEVEL: usize = 0;
pub(in native) static mut FNAME_FNAME: usize = 0;

const NAMES: [&str; 16] = [
    "^AMyCharacter::ForcedUnCrouch()",
    "^FSlateApplication::Tick()",
    "^FSlateApplication::OnKeyDown(int, unsigned int, bool)",
    "^FSlateApplication::OnKeyUp(int, unsigned int, bool)",
    "^FSlateApplication::OnRawMouseMove(int, int)",
    "^AController::GetControlRotation()",
    "^UEngine::UpdateTimeAndHandleMaxTickRate()",
    "^AMyCharacter::Tick(float)",
    "^FApp::DeltaTime",
    "^FMemory::Malloc(unsigned long, unsigned int)",
    "^FMemory::Free(void*)",
    "^UGameInstance::StartRecordingReplay(FString const&, FString const&, TArray<FString, FDefaultAllocator> const&)",
    "^UGameInstance::StopRecordingReplay()",
    "^UGameInstance::PlayReplay(FString const&, UWorld*, TArray<FString, FDefaultAllocator> const&)",
    "^UGameplayStatics::OpenLevel(UObject const*, FName, bool, FString)",
    "^FName::complete object constructor(wchar_t const*, EFindName)",
];

pub(in native) fn init() {
    let addrs: HashMap<_, _> = dynsym::iter(env::current_exe().unwrap()).into_iter()
        .filter_map(|(name, addr)| NAMES.iter()
            .find(|&&pattern| {
                if pattern.starts_with('^') {
                    name.starts_with(&pattern[1..])
                } else {
                    name.contains(pattern)
                }
            })
            .map(|&name| (name, addr)))
        .collect();
    log!("{:?}", addrs);
    unsafe {
        AMYCHARACTER_FORCEDUNCROUCH = *addrs.get(NAMES[0]).unwrap();
        log!("found AMyCharacter::execForcedUnCrouch: {:#x}", AMYCHARACTER_FORCEDUNCROUCH);
        FSLATEAPPLICATION_TICK = *addrs.get(NAMES[1]).unwrap();
        log!("found FSlateApplication::Tick: {:#x}", FSLATEAPPLICATION_TICK);
        FSLATEAPPLICATION_ONKEYDOWN = *addrs.get(NAMES[2]).unwrap();
        log!("found FSlateApplication::OnKeyDown: {:#x}", FSLATEAPPLICATION_ONKEYDOWN);
        FSLATEAPPLICATION_ONKEYUP = *addrs.get(NAMES[3]).unwrap();
        log!("found FSlateApplication::OnKeyUp: {:#x}", FSLATEAPPLICATION_ONKEYUP);
        FSLATEAPPLICATION_ONRAWMOUSEMOVE = *addrs.get(NAMES[4]).unwrap();
        log!("found FSlateApplication::OnRawMouseMove: {:#x}", FSLATEAPPLICATION_ONRAWMOUSEMOVE);
        ACONTROLLER_GETCONTROLROTATION = *addrs.get(NAMES[5]).unwrap();
        log!("found AController::GetControlRotation: {:#x}", ACONTROLLER_GETCONTROLROTATION);
        UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE = *addrs.get(NAMES[6]).unwrap();
        log!("found UEngine::UpdateTimeAndHandleMaxTickRate: {:#x}", UENGINE_UPDATETIMEANDHANDLEMAXTICKRATE);
        AMYCHARACTER_TICK = *addrs.get(NAMES[7]).unwrap();
        log!("found AMyCharacter::Tick: {:#x}", AMYCHARACTER_TICK);
        FAPP_DELTATIME = *addrs.get(NAMES[8]).unwrap();
        log!("found FApp::DeltaTime: {:#x}", FAPP_DELTATIME);
        FMEMORY_MALLOC = *addrs.get(NAMES[9]).unwrap();
        log!("found {}: {:#x}", NAMES[9], FMEMORY_MALLOC);
        FMEMORY_FREE = *addrs.get(NAMES[10]).unwrap();
        log!("found {}: {:#x}", NAMES[10], FMEMORY_FREE);
        UGAMEINSTANCE_STARTRECORDINGREPLAY = *addrs.get(NAMES[11]).unwrap();
        log!("found {}: {:#x}", NAMES[11], UGAMEINSTANCE_STARTRECORDINGREPLAY);
        UGAMEINSTANCE_STOPRECORDINGREPLAY = *addrs.get(NAMES[12]).unwrap();
        log!("found {}: {:#x}", NAMES[12], UGAMEINSTANCE_STOPRECORDINGREPLAY);
        UGAMEINSTANCE_PLAYREPLAY = *addrs.get(NAMES[13]).unwrap();
        log!("found {}: {:#x}", NAMES[13], UGAMEINSTANCE_PLAYREPLAY);
        UGAMEPLAYSTATICS_OPENLEVEL = *addrs.get(NAMES[14]).unwrap();
        log!("found {}: {:#x}", NAMES[14], UGAMEPLAYSTATICS_OPENLEVEL);
        FNAME_FNAME = *addrs.get(NAMES[15]).unwrap();
        log!("found {}: {:#x}", NAMES[15], FNAME_FNAME);
    }
}

pub(in native) fn make_rw(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_WRITE); }
}

pub(in native) fn make_rx(addr: usize) {
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    unsafe { libc::mprotect(page, 0x1000, PROT_READ | PROT_EXEC); }
}
