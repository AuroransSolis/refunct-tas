use native::ue::*;

#[repr(C)]
pub struct FRenderCommandFence {
    completion_event: FGraphEventRef, // 0x000
} // 0x008
