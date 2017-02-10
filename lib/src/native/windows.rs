use std::slice;
use std::ptr::null;

use winapi::{self, c_void};
use winapi::winnt::{PAGE_READWRITE, PAGE_EXECUTE_READ};
use kernel32::{VirtualProtect, GetModuleHandleA};
use byteorder::{WriteBytesExt, LittleEndian};

use consts;
use native::SLATEAPP;
use statics::Static;

// https://www.unknowncheats.me/forum/general-programming-and-reversing/123333-demo-pure-rust-internal-coding.html
#[no_mangle]
// Entry Point
pub extern "stdcall" fn DllMain(module: u32, reason: u32, reserved: *mut c_void) {
    match reason {
        1 => ::initialize(),
        _ => ()
    }
}

lazy_static! {
    static ref SLATEAPP_START: Static<[u8; 7]> = Static::new();
    static ref UNCROUCH_START: Static<[u8; 12]> = Static::new();
    static ref BASE: usize = unsafe { GetModuleHandleA(null()) as usize };
}

static mut FSLATEAPPLICATION_TICK: usize = 0;

pub fn init() {
    let base = &*BASE;
    log!("Got Base address: {:#x}", base);
    unsafe {
        FSLATEAPPLICATION_TICK = base + consts::FSLATEAPPLICATION_TICK;
    }
}

macro_rules! alignstack_pre {
    () => {{
        asm!(r"
            push ebp
            mov ebp, esp
            and esp, 0xfffffffffffffff0
        " :::: "intel");
    }}
}
macro_rules! alignstack_post {
    () => {{
        asm!(r"
            mov esp, ebp
            pop ebp
        " :::: "intel");
    }}
}

pub fn make_rw(addr: usize) {
    log!("make_rw: {:#x}", addr);
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_READWRITE, &mut out); }
}

pub fn make_rx(addr: usize) {
    log!("make_rx: {:#x}", addr);
    let page = addr & !0xfff;
    let page = page as *mut c_void;
    let mut out = 0;
    unsafe { VirtualProtect(page, 0x1000, PAGE_EXECUTE_READ, &mut out); }
}

pub fn hook_slateapp() {
    log!("Hooking FSlateApplication::Tick");
    let addr = &*BASE + consts::FSLATEAPPLICATION_TICK;
    make_rw(addr);
    let hook_fn = get_slateapp as *const () as usize;
    let mut tick = unsafe { slice::from_raw_parts_mut(addr as *mut u8, 7) }; 
    let mut saved = [0u8; 7];
    saved[..].copy_from_slice(tick);
    SLATEAPP_START.set(saved);
    log!("orig tick: {:?}", tick);
    // mov eax, addr
    tick[0] = 0xb8;
    (&mut tick[1..5]).write_u32::<LittleEndian>(hook_fn as u32).unwrap();
    // jmp rax
    tick[5..].copy_from_slice(&[0xff, 0xe0]);
    log!("Injected Code: {:?}", tick);
    make_rx(addr);
    log!("FSlateApplication::Tick successfully hooked");
}

#[inline(never)]
#[no_mangle]
#[naked]
unsafe extern fn get_slateapp() -> ! {
    // push argument
    asm!("push ecx" :::: "intel");
    // call interceptor
    asm!("call $0
    " :: "i"(save_slateapp as usize) :: "intel");
    // restore everything and jump to original function
    asm!(r"
        pop ecx
        jmp eax
    ":: "{eax}"(FSLATEAPPLICATION_TICK) :: "intel");
    ::std::intrinsics::unreachable()
}

#[inline(never)]
#[no_mangle]
extern fn save_slateapp(this: usize) {
    log!("save_slateapp");
    let addr = &*BASE + consts::FSLATEAPPLICATION_TICK;
    make_rw(addr);
    SLATEAPP.set(this);
    let mut tick = unsafe { slice::from_raw_parts_mut(addr as *mut _, 7) }; 
    tick.copy_from_slice(&*SLATEAPP_START.get());
    make_rx(addr);
    log!("Got FSlateApplication: {:#x}", this);
}

//pub fn hook_newgame() {
    //log!("Hooking AMyCharacter::execForcedUnCrouch");
    //make_rw(consts::AMYCHARACTER_EXECFORCEDUNCROUCH);
    //let hook_fn = new_game as *const () as usize;
    //let mut tick = unsafe { slice::from_raw_parts_mut(consts::AMYCHARACTER_EXECFORCEDUNCROUCH as *mut u8, 12) }; 
    //let mut saved = [0u8; 12];
    //saved[..].copy_from_slice(tick);
    //UNCROUCH_START.set(saved);
    //log!("orig execforceduncrouch: {:?}", tick);
    //// mov rax, addr
    //tick[..2].copy_from_slice(&[0x48, 0xb8]);
    //(&mut tick[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
    //// jmp rax
    //tick[10..].copy_from_slice(&[0xff, 0xe0]);
    //log!("Injected Code: {:?}", tick);
    //make_rx(consts::AMYCHARACTER_EXECFORCEDUNCROUCH);
    //log!("AMyCharacter::execForcedUnCrouch successfully hooked");
//}

//fn restore_newgame() {
    //log!("Restoring AMyCharacter::execForcedUnCrouch");
    //make_rw(consts::AMYCHARACTER_EXECFORCEDUNCROUCH);
    //let mut tick = unsafe { slice::from_raw_parts_mut(consts::AMYCHARACTER_EXECFORCEDUNCROUCH as *mut u8, 12) }; 
    //tick[..].copy_from_slice(&*UNCROUCH_START.get());
    //make_rx(consts::AMYCHARACTER_EXECFORCEDUNCROUCH);
    //log!("AMyCharacter::execForcedUnCrouch successfully restored");
//}

//#[naked]
//unsafe extern fn new_game() -> ! {
    //// push arguments
    //asm!("push rdi" :::: "intel");
    //asm!("push rsi" :::: "intel");
    //alignstack_pre!();
    //// call interceptor
    //asm!("call rax" :: "{rax}"(::native::new_game as usize) :: "intel");
    //// restore original function
    //asm!("call rax" :: "{rax}"(restore_newgame as usize) :: "intel");
    //alignstack_post!();
    //// restore registers
    //asm!(r"
        //pop rsi
        //pop rdi
    //" :::: "intel");
    //alignstack_pre!();
    //// call original function
    //asm!("call rax" :: "{rax}"(consts::AMYCHARACTER_EXECFORCEDUNCROUCH) :: "intel");
    //alignstack_post!();
    //// save rax (return value of original function)
    //asm!("push rax" :::: "intel");
    //alignstack_pre!();
    //// hook method again
    //asm!("call rax" :: "{rax}"(hook_newgame as usize) :: "intel");
    //alignstack_post!();
    //// restore rax
    //asm!("pop rax" :::: "intel");
    //// return to the original caller
    //asm!("ret" :::: "intel");
    //::std::intrinsics::unreachable()
//}

//pub fn hook_tick() {
    //::std::thread::sleep(::std::time::Duration::from_secs(5));
    //log!("Hooking FEngineLoop::Tick");
    //make_rw(consts::FENGINELOOP_TICK_AFTER_UPDATETIME);
    //let hook_fn = tick as *const () as usize;
    //let mut bytes = unsafe { slice::from_raw_parts_mut(consts::FENGINELOOP_TICK_AFTER_UPDATETIME as *mut u8, 14) };
    //// mov rax, addr
    //bytes[..2].copy_from_slice(&[0x48, 0xb8]);
    //(&mut bytes[2..10]).write_u64::<LittleEndian>(hook_fn as u64).unwrap();
    //// jmp rax
    //bytes[10..12].copy_from_slice(&[0xff, 0xe0]);
    //// nop
    //bytes[12] = 0x90;
    //bytes[13] = 0x90;
    //log!("Injected Code: {:?}", bytes);
    //make_rx(consts::FENGINELOOP_TICK_AFTER_UPDATETIME);
    //log!("FEngineLoop::Tick hooked successfully");
//}

//#[naked]
//unsafe extern fn tick() -> ! {
    //// we are inside a function, so we need to push everything
    //asm!(r"
        //push rax
        //push rbx
        //push rcx
        //push rdx
        //push rsi
        //push rdi
        //push rbp
        //push r8
        //push r9
        //push r10
        //push r11
        //push r12
        //push r13
        //push r14
        //push r15
        //sub rsp, 0x80
        //movdqu [rsp+0x70], xmm0
        //movdqu [rsp+0x60], xmm1
        //movdqu [rsp+0x50], xmm2
        //movdqu [rsp+0x40], xmm3
        //movdqu [rsp+0x30], xmm4
        //movdqu [rsp+0x20], xmm5
        //movdqu [rsp+0x10], xmm6
        //movdqu [rsp], xmm7
    //" :::: "intel");

    //alignstack_pre!();
    //// call our function
    //asm!("call rax" :: "{rax}"(::native::tick_intercept as usize) :: "intel");
    //alignstack_post!();

    //// restore all registers
    //asm!(r"
        //movdqu xmm7, [rsp]
        //movdqu xmm6, [rsp+0x10]
        //movdqu xmm5, [rsp+0x20]
        //movdqu xmm4, [rsp+0x30]
        //movdqu xmm3, [rsp+0x40]
        //movdqu xmm2, [rsp+0x50]
        //movdqu xmm1, [rsp+0x60]
        //movdqu xmm0, [rsp+0x70]
        //add rsp, 0x80
        //pop r15
        //pop r14
        //pop r13
        //pop r12
        //pop r11
        //pop r10
        //pop r9
        //pop r8
        //pop rbp
        //pop rdi
        //pop rsi
        //pop rdx
        //pop rcx
        //pop rbx
        //pop rax
    //" :::: "intel");
    //// execute the 3 instructions which we overwrote
    //asm!(r"
        //mov rdi, [$0]
        //mov rax, [rdi]
        //call [rax+0x60]
    //" :: "i"(consts::GMALLOC) :: "intel");
    //// jump to original tick function after our hook
    //asm!(r"
        //mov rax, $0
        //jmp rax
    //" :: "i"(consts::FENGINELOOP_TICK_AFTER_UPDATETIME + 14) :: "intel");
    //::std::intrinsics::unreachable()
//}
