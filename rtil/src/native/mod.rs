macro_rules! ptr_eq {
    ($var:ident, $($field:ident).*, $cmp:expr) => {
        assert_eq!(&(*$var).$($field).* as *const _ as usize - $var as usize, $cmp);
    };
    ($var:ident, $($field:ident).*, $cmp:expr, $args:tt) => {
        assert_eq!(&(*$var).$($field).* as *const _ as usize - $var as usize, $cmp, $args);
    }
}

#[cfg(unix)] #[macro_use] mod linux;
#[cfg(windows)] #[macro_use] mod windows;
pub mod ue;
mod character;
mod controller;
mod newgame;
mod slateapp;
mod tick;
mod app;
mod memory;
mod world;
mod gameinstance;
mod actor;
mod pawn;
mod player;
mod gamestate;
mod level;
mod scenecomponent;
mod actorcomponent;
mod charactermovementcomponent;
mod pawnmovementcomponent;
mod navmovementcomponent;
mod movementcomponent;

#[cfg(unix)] use self::linux::*;
#[cfg(windows)] use self::windows::*;

#[cfg(unix)] pub use self::linux::INITIALIZE_CTOR;
#[cfg(windows)] pub use self::windows::DllMain;
pub use self::character::AMyCharacter;
pub use self::controller::AController;
pub use self::slateapp::FSlateApplication;
pub use self::app::FApp;
pub use self::memory::FMemory;

pub fn init() {
    #[cfg(windows)] windows::init();
    #[cfg(unix)] linux::init();
    slateapp::hook();
    newgame::hook();
    tick::hook();
    controller::hook();
//    character::hook();
}
