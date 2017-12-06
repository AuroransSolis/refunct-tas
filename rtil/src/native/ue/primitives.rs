#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Bool32(u32);

impl From<Bool32> for bool {
    fn from(b: Bool32) -> Self {
        match b.0 {
            0 => false,
            _ => true,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct Bool8(u8);

impl From<Bool8> for bool {
    fn from(b: Bool8) -> Self {
        match b.0 {
            0 => false,
            _ => true,
        }
    }
}

pub type Unk1 = [u8; 1];
pub type Unk2 = [u8; 2];
pub type Unk4 = [u8; 4];
pub type Unk8 = [u8; 8];
