use native::ue::*;
use native::navmovementcomponent::UNavMovementComponent;
use native::pawn::APawn;

pub struct UPawnMovementComponent {
    base: UNavMovementComponent,
    pawn_owner: *const APawn<()>,
}
