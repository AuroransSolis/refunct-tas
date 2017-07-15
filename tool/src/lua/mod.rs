use std::rc::Rc;
use std::cell::RefCell;

use hlua::{self, Lua};

use tas::{Tas, Response};
use config::Config;

pub fn init_tas(lua: &mut Lua, outer: Rc<RefCell<Tas>>, config: Config) {
    let tas = outer.clone();
    lua.set("__stop", hlua::function0(move || {
        tas.borrow_mut().stop().expect("error stopping")
    }));

    let tas = outer.clone();
    lua.set("__step", hlua::function0(move || {
        match tas.borrow_mut().step().expect("error stepping") {
            Response::NewGame => panic!("Got NewGame but expected PlayerStats"),
            Response::Stopped(stats) => stats
        }
    }));

    let tas = outer.clone();
    let cfg = config.clone();
    lua.set("__press_key", hlua::function1(move |key: String| {
        let key = to_key(&key, &cfg);
        tas.borrow_mut().press_key(key).expect("error pressing key");
    }));

    let tas = outer.clone();
    lua.set("__release_key", hlua::function1(move |key: String| {
        let key = to_key(&key, &config);
        tas.borrow_mut().release_key(key).expect("error releasing key");
    }));

    let tas = outer.clone();
    lua.set("__move_mouse", hlua::function2(move |x: i32, y: i32| {
        tas.borrow_mut().move_mouse(x, y).expect("error moving mouse");
    }));

    let tas = outer.clone();
    lua.set("__set_delta", hlua::function1(move |delta: f64| {
        tas.borrow_mut().set_delta(delta).expect("error getting player stats")
    }));

    let tas = outer.clone();
    lua.set("__set_location", hlua::function3(move |x: f32, y: f32, z: f32| {
        tas.borrow_mut().set_location(x, y, z).expect("error setting location")
    }));

    let tas = outer.clone();
    lua.set("__set_rotation", hlua::function3(move |pitch: f32, yaw: f32, roll: f32| {
        tas.borrow_mut().set_rotation(pitch, yaw, roll).expect("error setting rotation")
    }));

    let tas = outer.clone();
    lua.set("__set_velocity", hlua::function3(move |x: f32, y: f32, z: f32| {
        tas.borrow_mut().set_velocity(x, y, z).expect("error setting location")
    }));

    let tas = outer.clone();
    lua.set("__wait_for_new_game", hlua::function0(move || {
        tas.borrow_mut().wait_for_new_game().expect("error waiting for new game")
    }));
}

fn to_key(key: &str, cfg: &Config) -> i32 {
    match key {
        "forward" => cfg.forward.into(),
        "backward" => cfg.backward.into(),
        "left" => cfg.left.into(),
        "right" => cfg.right.into(),
        "jump" => cfg.jump.into(),
        "crouch" => cfg.crouch.into(),
        "menu" => cfg.menu.into(),
        s => panic!("Unknown key {}", s)
    }
}
