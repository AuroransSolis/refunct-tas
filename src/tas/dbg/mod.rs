#[cfg(unix)]
mod linux;
#[cfg(windows)]
mod windows;
mod pidof;

pub use pidof::pidof;
#[cfg(unix)]
pub use linux::Debugger;
#[cfg(windows)]
pub use windows::Debugger;

use std::io::Result;

pub trait Dbg {
    type BpIdent: Display;
    type AddrIdent: Display;
    
    fn new(pid: u32) -> Result<Self>;
    fn send_cmd(&mut self, cmd: &str) -> Result<String>;
    fn breakpoint_set(&mut self, addr: usize) -> Result<BpIdent>;
    fn breakpoint_disable(&mut self, bp: &BpIdent) -> Result<()>;
    fn breakpoint_enable(&mut self, bp: &BpIdent) -> Result<()>;
    fn breakpoint_delete(&mut self, bp: &BpIdent) -> Result<()>;
    fn cont(&mut self) -> Result<()>;
    fn save_rdi(&mut self) -> Result<AddrIdent>;
}
