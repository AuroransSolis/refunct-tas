use std::ptr;
use std::mem;
use std::marker::PhantomData;
use std::ops::Index;

use native::FMemory;
use native::FNAME_FNAME;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FRotator {
    pitch: f32,
    yaw: f32,
    roll: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FVector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type FVector_NetQuantize10 = FVector;
pub type FVector_NetQuantize100 = FVector;

#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct FVector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FIntVector {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FQuat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FTransform {
    pub rotation: FQuat,
    pub translation: FVector,
    pub scale3d: FVector,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FVector2D {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FRotationConversionCache {
    cached_quat: FQuat,
    cached_rotator: FRotator,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FBox {
    min: FVector, // 0x000
    max: FVector, // 0x00c
    is_valid: Bool8, // 0x018
} // 0x019

#[repr(C)]
#[derive(Debug, Clone)]
pub struct FBoxSphereBounds {
    origin: FVector, // 0x000
    box_extent: FVector, // 0x00c
    sphere_radius: f32, // 0x018
} // 0x01c

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FForceFeedbackValues {
    left_large: f32, // 0x000
    left_small: f32, // 0x004
    right_large: f32, // 0x008
    right_small: f32, // 0x00c
} // 0x010

#[repr(C)]
#[derive(Debug)]
pub struct TArray<T> {
    ptr: *mut T,
    len: i32,
    capacity: i32,
}

impl<T> TArray<T> {
    pub fn new() -> TArray<T> {
        TArray::with_capacity(0)
    }

    pub fn with_capacity(cap: usize) -> TArray<T> {
        let ptr = if cap > 0 {
            unsafe { FMemory::malloc(cap * mem::size_of::<T>()) as *mut T }
        } else {
            ptr::null_mut()
        };
        TArray {
            ptr,
            len: 0,
            capacity: cap as i32,
        }
    }

    pub fn push(&mut self, t: T) {
        assert!(self.len  < self.capacity);
        unsafe { *self.ptr.offset(self.len as isize) = t };
        self.len += 1;
    }

    pub fn capacity(&self) -> i32 {
        self.capacity
    }
    pub fn len(&self) -> i32 {
        self.len
    }
}

impl<T> Index<usize> for TArray<T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        assert!((index as i32) < self.len, "index out of range");
        unsafe {
            &*self.ptr.offset(index as isize)
        }
    }
}

impl<T> Index<i32> for TArray<T> {
    type Output = T;

    fn index(&self, index: i32) -> &T {
        assert!(index >= 0, "negative index");
        self.index(index as usize)
    }
}

/// Null-terminated utf-32 array
pub struct FString(TArray<char>);

impl FString {
    pub fn new() -> FString {
        FString(TArray::new())
    }

    pub unsafe fn as_ptr(&self) -> *const char {
        self.0.ptr
    }
}

impl<S: AsRef<str>> From<S> for FString {
    fn from(s: S) -> Self {
        assert_eq!(::std::mem::size_of::<char>(), 4);
        let s = s.as_ref();
        let len = s.chars().count();
        let mut arr = TArray::with_capacity(len + 1);
        for c in s.chars() {
            arr.push(c)
        }
        arr.push(0 as char);

        FString(arr)
    }
}

impl<T> Drop for TArray<T> {
    fn drop(&mut self) {
        unsafe {
            FMemory::free(self.ptr as *mut ())
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct FName {
    number: u64,
}

impl<T: Into<FString>> From<T> for FName {
    fn from(s: T) -> FName {
        let s = s.into();
        let mut name = FName {
            number: 0,
        };
        unsafe {
            let fun: extern "C" fn(*mut FName, *const char, u64) -> u64
                = mem::transmute(FNAME_FNAME);
            fun(&mut name as *mut FName, s.as_ptr(), 1);
        }
        name
    }
}

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

#[repr(C)]
pub struct UObjectBase<B> {
    vtable: *const (), // 0x000
    object_flags: EObjectFlags, // 0x008
    internal_index: i32, // 0x00c
    class_private: *const UClass, // 0x0x010
    name_private: FName, // 0x018 / 0x010
    outer_private: *const B, // 0x020 / 0x018
    vtable2: *const (), // 0x028 / 0x01c
} // 0x030 / 0x020

pub struct UObject<B> {
    base: UObjectBase<B>
} // 0x030 / 0x020

#[repr(C)]
pub enum ERHIFeatureLevelType {
    ES2,
    #[allow(non_camel_case_types)]
    ES3_1,
    SM4,
    SM5,
    Num
}

#[repr(C)]
pub struct TWeakObjectPtr<T> {
    object_index: i32,
    object_serial_number: i32,
    _t: PhantomData<T>,
}

#[repr(C)]
pub struct FGuid {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

#[repr(C)]
pub struct TSet<T> {
    _t: PhantomData<T>,
}

#[repr(C)]
pub struct TSharedPtr<T> {
    ptr: *const T,
    shared_reference_count: *const FReferenceControllerBase,
}

#[repr(C)]
pub struct TLinkedList<T> {
    next_link: *const T,
    prev_link: *const *const T,
    element: T,
}

// may be wrong
#[repr(C)]
pub struct FSceneViewStateReference {
    reference: *const FSceneViewStateInterface,
    global_list_link: TLinkedList<*const FSceneViewStateReference>,
}

#[repr(C)]
pub struct FTimerHandle {
    handle: u64,
}

#[repr(C)]
pub struct FUrl {
    protocol: FString, // 0x000
    host: FString, // 0x010
    port: i32, // 0x020
    map: FString, // 0x028
    redirect_url: FString, // 0x038
    op: TArray<FString>, // 0x048
    portal: FString, // 0x058
    valid: i32, // 0x068
} // 0x06c

pub type FGraphEventRef = TRefCountPtr<FGraphEvent>;

/// Implements ref counting for classes implementing AddRef / Release
#[repr(C)]
pub struct TRefCountPtr<T> {
    reference: *const T,
}

#[repr(C)]
pub struct TSharedRef<T> {
    object: *const T,
}

// Stubs

#[repr(C)]
pub struct TSubclassOf<C> {
    class: *const UClass,
    _c: PhantomData<C>,
}

#[repr(C)]
pub struct TUniquePtr<T> {
    _t: PhantomData<T>,
}

#[repr(C)]
pub struct TTraceThreadData<T> {
    _t: PhantomData<T>,
}

#[repr(C)]
pub struct TEnumAsByte<T> {
    value: u8,
    _t: PhantomData<T>,
}

#[repr(C)]
pub struct TMap<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

pub type Unk1 = [u8; 1];
pub type Unk2 = [u8; 2];
pub type Unk4 = [u8; 4];
pub type Unk8 = [u8; 8];

pub struct UClass;
pub struct UPackage;
pub struct ULineBatchComponent;
pub struct AGameNetworkManager;
pub struct UPhysicsCollisionHandler;
pub struct UNetDriver;
pub struct ULevelStreaming;
pub struct UDemoNetDriver;
pub struct AParticleEventManager;
pub struct APhysicsVolume;
pub struct UNavigationSystem;
pub struct AGameModeBase;
pub struct AGameMode;
pub struct UAISystemBase;
pub struct UAvoidanceManager;
pub struct FLevelCollection;
pub struct UMaterialParameterCollectionInstance;
pub struct UCanvas;
pub struct FScene;
pub struct APlayerController;
pub struct ACameraActor;
pub struct FPhysScene;
pub struct UGameEngine;
pub struct UOnlineSession;
pub struct FOnPreClientTravel;
pub struct FDelegateHandle;
pub struct FTimerManager;
pub struct FLatentActionManager;
pub struct USaveGame;
pub struct UUserWidget;
pub struct FUniqueNetId;
pub struct UGameViewportClient;
pub struct FReferenceControllerBase;
pub struct FSceneViewStateInterface;
// actually bitflags
#[repr(C)]
pub enum EObjectFlags {
    // rust can't represent unsized types as sized types
    __LEL,
}
pub struct UInputComponent;
pub struct AMatineeActor;
pub struct UChildActorComponent;
pub struct FTickPrerequisite;
pub struct FTickTaskLevel;
pub struct UBillboardComponent;
pub struct ASpectatorPawn;
pub struct APlayerState;
pub struct USkeletalMeshComponent;
pub struct UCapsuleComponent;
pub struct UPrimitiveComponent;
pub struct FSimulatedRootMotionReplicatedMove;
pub struct UAnimMontage;
pub struct FRootMotionSource;
pub struct FTraceDatum;
pub struct FOverlapDatum;
pub struct UModel;
pub struct UModelComponent;
pub struct ULevelActorContainer;
pub struct ULevelScriptBlueprint;
pub struct ALevelScriptActor;
pub struct ANavigationObjectBase;
pub struct UNavigationDataChunk;
pub struct FPrecomputedLightVolume;
pub struct FPrecomputedVolumetricLightmap;
pub struct UMapBuildDataRegistry;
pub struct ALevelBounds;
pub struct AInstanceFoliageActor;
pub struct AWorldSettings;
pub struct UAssetUserData;
pub struct FPendingAutoReceiveInputActor;
pub struct FPrecomputedVisibilityBucket;
pub struct FColor;
pub struct FGraphEvent;
pub struct UPlayer;
pub struct UInterpTrackInstDirector;
pub struct AHud;
pub struct APlayerCameraManager;
pub struct UCheatManager;
pub struct UPlayerInput;
pub struct FActiveForceFeedbackEffect;
pub struct FDynamicForceFeedbackDetails;
pub struct UNetConnection;
pub struct FKey;
#[repr(C)]
pub enum EMouseCursorType {
    __LEL,
}
#[repr(C)]
pub enum ECollisionChannel {
    __LEL,
}
pub struct SVirtualJoystick;
pub struct FActiveHapticFeedbackEffect;
pub struct UTouchInterface;
pub struct FSimpleMemberReference;
pub struct FScopedMovementUpdate;
pub struct ANavigationData;
pub struct UPathFollowingComponent;
pub struct FSavedMove_Character;
pub struct FCharacterReplaySample;
