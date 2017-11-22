use native::ue::{FString, FName};
use native::UGameInstance;

pub struct UWorld;

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

pub struct UGameplayStatics;

impl UGameplayStatics {
    pub fn open_level<S: Into<FName>>(name: S) {
        unsafe {
            let fun: extern "C" fn(*const UWorld, FName, bool, *const FString)
                = unsafe { ::std::mem::transmute(::native::linux::UGAMEPLAYSTATICS_OPENLEVEL) };
            let name = name.into();
            let options = FString::new();
            fun(*(::native::linux::GWORLD as *const *const UWorld), name, true, &options as *const FString);
        }
    }
}