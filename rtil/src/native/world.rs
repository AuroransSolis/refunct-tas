pub struct UWorld;

use native::UGameInstance;

impl UWorld {
    pub fn game_instance() -> &'static UGameInstance {
        unsafe {
            let uworld_ptr = *(::native::linux::GWORLD as *const usize);
            let ugameinstance_ptr_ptr = uworld_ptr + 0x138;
            let ugameinstance_ptr = *(ugameinstance_ptr_ptr as *const *const UGameInstance);
            let ugameinstance_ref = ::std::mem::transmute(ugameinstance_ptr);
            ugameinstance_ref
        }
    }
}