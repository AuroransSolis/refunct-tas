mod listener;
mod stream_read;
mod stream_write;
mod lua;
pub mod ue;

use std::sync::mpsc;

pub fn start() {
    let (stream_lua_tx, stream_lua_rx) = mpsc::channel();
    let (lua_stream_tx, lua_stream_rx) = mpsc::channel();
    let (lua_ue_tx, lua_ue_rx) = mpsc::channel();
    let (ue_lua_tx, ue_lua_rx) = mpsc::channel();
    listener::run(stream_lua_tx, lua_stream_rx).unwrap();
    lua::run(stream_lua_rx, lua_stream_tx, lua_ue_tx, ue_lua_rx);
    ue::run(lua_ue_rx, ue_lua_tx);
}

#[derive(Debug, PartialEq, Eq)]
pub enum ListenerToStream {
    KillYourself,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StreamToListener {
    ImDead,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StreamToLua {
    Start(String),
    Stop,
    Config(Config),
    WorkingDir(String),
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Config {
    forward: i32,
    backward: i32,
    left: i32,
    right: i32,
    jump: i32,
    crouch: i32,
    menu: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LuaToStream {
    Print(String),
    MiDone,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LuaToUe {
    Stop,
    AdvanceFrame,
    // we need to execute events on the main loop, because possible played audio
    // needs to access thread-local storage
    PressKey(i32),
    ReleaseKey(i32),
    MoveMouse(i32, i32),
    Resume,
}

#[derive(Debug, PartialEq, Eq)]
pub enum UeToLua {
    Tick,
    NewGame,
}
