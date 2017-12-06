use std::marker::PhantomData;

#[repr(C)]
pub struct TSet<T> {
    _t: PhantomData<T>,
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
pub struct TMap<K, V> {
    _k: PhantomData<K>,
    _v: PhantomData<V>,
}

// actually bitflags
#[repr(C)]
pub enum EObjectFlags {
    // rust can't represent unsized types as sized types
    __LEL,
}

#[repr(C)]
pub enum EMouseCursorType {
    __LEL,
}

#[repr(C)]
pub enum ECollisionChannel {
    __LEL,
}

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
pub struct SVirtualJoystick;
pub struct FActiveHapticFeedbackEffect;
pub struct UTouchInterface;
pub struct FSimpleMemberReference;
pub struct FScopedMovementUpdate;
pub struct ANavigationData;
pub struct UPathFollowingComponent;
#[allow(non_camel_case_types)]
pub struct FSavedMove_Character;
pub struct FCharacterReplaySample;
