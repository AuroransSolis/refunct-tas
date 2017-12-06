use native::ue::*;

#[repr(C)]
pub struct FTimerHandle {
    handle: u64,
}

#[repr(C)]
pub enum ENetRole {
    None,
    SimulatedProxy,
    AutonomousProxy,
    Authority,
    Max,
}

#[repr(C)]
pub struct FRepMovement {
    linear_velocity: FVector,
    angular_velocity: FVector,
    location: FVector,
    rotation: FRotator,
    //    b_simulated_physics_sleep: Bool8,
    //    b_rep_physics: Bool8,
    bitfield: Bool8,
    location_quantization_level: EVectorQuantization,
    velocity_quantization_level: EVectorQuantization,
    rotation_quantization_level: EVectorQuantization,
}

#[repr(C)]
pub struct FRepAttachment {
    attach_parent: *const AActor<()>,
    location_offset: FVector_NetQuantize100,
    relative_scale_3d: FVector_NetQuantize100,
    rotation_offset: FRotator,
    attach_socket: FName,
    attach_component: *const USceneComponent,
}

#[repr(u8)]
pub enum EVectorQuantization {
    RoundWholeNumber,
    RoundOneDecimal,
    RoundTwoDecimals,
}

#[repr(C)]
pub enum ENetDormancy {
    Never,
    Awake,
    DormantAll,
    DormantPartial,
    Initial,
    Max,
}

#[repr(C)]
pub enum EAutoReceiveInputType {
    Disabled,
    Player0,
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
    Player7,
}

#[repr(u8)]
pub enum ESpawnActorCollisionHandlingMethod {
    Undefined,
    AlwaysSpawn,
    AdjustIfPossibleButAlwaysSpawn,
    AdjustIfPossibleButDontSpawnIfColliding,
    DontSpawnIfColliding,
}

#[repr(u8)]
pub enum ENetworkSmoothingMode {
    Disabled,
    Linear,
    Exponential,
    Replay,
}

#[repr(C)]
pub enum EMovementMode {
    None,
    Walking,
    NavWalking,
    Falling,
    Swimming,
    Flying,
    Custom,
    MAX,
}

#[repr(C)]
pub struct FHitResult {
    normal: FVector4,
    time: f32,
    item: i32,
}

#[repr(C)]
pub enum EComponentMobilityType {
    Static,
    Stationary,
    Movable,
}

