use native::ue::*;
use native::navmovementcomponent::UNavMovementComponent;
use native::pawn::APawn;

pub struct UPawnMovementComponent {
    pub base: UNavMovementComponent,
    pawn_owner: *const APawn<()>,
}
