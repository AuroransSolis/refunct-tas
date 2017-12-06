use native::ue::*;

pub struct UPawnMovementComponent {
    pub base: UNavMovementComponent,
    pawn_owner: *const APawn<()>,
}
