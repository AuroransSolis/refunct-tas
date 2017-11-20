use std::ptr;

use native::ue::{FString, TArray};
use native::{UWorld};

pub struct UGameInstance;

impl UGameInstance {
    pub fn start_recording_replay<S: Into<FString>>(&self, name: S) {
        let fun: extern "C" fn(*const UGameInstance, *const FString, *const FString, *const TArray<FString>)
            = unsafe { ::std::mem::transmute(::native::linux::UGAMEINSTANCE_STARTRECORDINGREPLAY) };
        let name = name.into();
        let friendly_name = FString::new();
        let additional_options: TArray<FString> = TArray::new();
        fun(self as *const UGameInstance, &name as *const FString,
            &friendly_name as *const FString, &additional_options as *const TArray<FString>)
    }

    pub fn stop_recording_replay(&self) {
        let fun: extern "C" fn(*const UGameInstance)
            = unsafe { ::std::mem::transmute(::native::linux::UGAMEINSTANCE_STOPRECORDINGREPLAY) };
        fun(self as *const UGameInstance)
    }

    pub fn play_replay<S: Into<FString>>(&self, name: S) {
        let fun: extern "C" fn(*const UGameInstance, *const FString, *const UWorld, *const TArray<FString>)
            = unsafe { ::std::mem::transmute(::native::linux::UGAMEINSTANCE_PLAYREPLAY) };
        let name = name.into();
        let additional_options: TArray<FString> = TArray::new();
        unsafe {
            fun(self as *const UGameInstance, &name as *const FString,
                ptr::null(), &additional_options as *const TArray<FString>)
        }
    }
}