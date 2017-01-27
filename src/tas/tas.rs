use error::*;
use config::Ingame;
use consts;
use tas::parser::Frame;
use tas::dbg::{Debugger, Dbg};

pub struct Tas {
    dbg: Debugger,
    fslateapplication: Debugger::AddrIdent,
    tickbp: Debugger::BpIdent,
    newgamebp: Debugger::BpIdent,
}

impl Tas {
    pub fn new(pid: u32) -> Result<Tas> {
        let dbg = Debugger::start(pid).chain_err(|| "Cannot start gdb")?;
        let slatebp = self.dbg.breakpoint_set(consts::FSLATEAPPLICATION_TICK)?;
        self.dbg.cont()?;
        let fslateapplication = self.dbg.save_rdi()?;
        self.dbg.breakpoint_disable(&slatebp)?;
        let tickbp = self.dbg.breakpoint_set(consts::FENGINELOOP_TICK_AFTER_UPDATETIME)?;
        let newgamebp = self.dbg.breakpoint_set(consts::AMYCHARACTER_EXECFORCEDUNCROUCH)?;
        self.dbg.breakpoint_disable(&newgamebp)?;
        self.dbg.breakpoint_disable(&tickbp)?;
        Ok(Tas {
            dbg: dbg,
            fslateapplication: fslateapplication,
            tickbp: tickbp,
            newgamebp: newgamebp,
        })
    }

    pub fn step(&mut self) -> Result<()> {
        // set delta float for smooth fps
        self.dbg.send_cmd(&format!("set {{double}} {:#} = {}", consts::APP_DELTATIME, 1f64 / 60f64))?;
        self.dbg.cont()?;
        Ok(())
    }

    pub fn press_key(&mut self, key: char) -> Result<()> {
        self.send_cmd(&format!("call ((void(*)(void*, int, int, int)){0})($fslateapplication,{1},{1},0)", consts::FSLATEAPPLICATION_ONKEYDOWN, key as u8)).map(|_| ())
    }

    pub fn release_key(&mut self, key: char) -> Result<()> {
        self.send_cmd(&format!("call ((void(*)(void*, int, int, int)){0})($fslateapplication,{1},{1},0)", consts::FSLATEAPPLICATION_ONKEYUP, key as u8)).map(|_| ())
    }

    pub fn move_mouse(&mut self, x: i32, y: i32) -> Result<()> {
        self.send_cmd(&format!("call ((void(*)(void*, int, int)){})($fslateapplication,{},{})", consts::FSLATEAPPLICATION_ONRAWMOUSEMOVE, x, y)).map(|_| ())
    }

    pub fn wait_for_new_game(&mut self) -> Result<()> {
        self.send_cmd("enable $newgamebp")?;
        self.send_cmd("c")?;
        self.send_cmd("disable $newgamebp").map(|_| ())
    }

    pub fn play(&mut self, frames: &Vec<Frame>, inputs: &Ingame) -> Result<()> {
        self.send_cmd("enable $tickbp")?;
        let mut last = Frame::default();
        for frame in frames {
            // new inputs
            if frame.forward && !last.forward {
                self.press_key(inputs.forward)?;
            }
            if frame.backward && !last.backward {
                self.press_key(inputs.backward)?;
            }
            if frame.left && !last.left {
                self.press_key(inputs.left)?;
            }
            if frame.right && !last.right {
                self.press_key(inputs.right)?;
            }
            if frame.jump && !last.jump {
                self.press_key(inputs.jump)?;
            }

            // old inputs
            if last.forward && !frame.forward {
                self.release_key(inputs.forward)?;
            }
            if last.backward && !frame.backward {
                self.release_key(inputs.backward)?;
            }
            if last.left && !frame.left {
                self.release_key(inputs.left)?;
            }
            if last.right && !frame.right {
                self.release_key(inputs.right)?;
            }
            if last.jump && !frame.jump {
                self.release_key(inputs.jump)?;
            }

            last = frame.clone();

            // press ESC
            if frame.esc {
                 self.press_key(0x1b as char)?;
            }

            // mouse movements
            if frame.mouse_x != 0 || frame.mouse_y != 0 {
                 self.move_mouse(frame.mouse_x, frame.mouse_y)?;
            }

            self.step()?;
        }
        self.send_cmd("disable $tickbp").map(|_| ())
    }
}

