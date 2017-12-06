use native::ue::*;

#[repr(C)]
pub struct AInfo<B> {
    base: AActor<B>,
    sprite_component: *const UBillboardComponent,
} // 0x388

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of() {
        use std::mem::size_of;
        assert_eq!(size_of::<AInfo<()>>(), 0x388);
    }
}
