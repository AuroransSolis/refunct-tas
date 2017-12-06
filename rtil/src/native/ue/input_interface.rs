#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FForceFeedbackValues {
    left_large: f32, // 0x000
    left_small: f32, // 0x004
    right_large: f32, // 0x008
    right_small: f32, // 0x00c
} // 0x010
