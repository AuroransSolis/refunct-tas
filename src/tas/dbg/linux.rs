use std::io::{Write, BufWriter, BufReader, BufRead, Result};
use std::process::{ChildStdin, ChildStdout};

use tas::dbg::Dbg;

pub struct Debugger {
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<ChildStdout>,
}

impl Dbg for Debugger {
    type BpIdent = String;

    fn new(pid: u32) -> Result<Debugger> {
        let mut child = process::Command::new("./cdb.exe")
            .arg(&format!("-p {}", pid))
            .stdout(process::Stdio::piped())
            .stdin(process::Stdio::piped())
            .stderr(process::Stdio::piped())
            .spawn()?;
        let mut result = Debugger {
            stdin: BufWriter::new(child.stdin.take().expect("broken stdin")),
            stdout: BufReader::new(child.stdout.take().expect("broken stdout")),
        };
        result.read_sequence()?;
        Ok(result)
    }

    fn send_cmd(&mut self, cmd: &str) -> Result<String> {
        if cmd.ends_with("\n") {
            write!(self.stdin, "{}", cmd)?;
        } else {
            writeln!(self.stdin, "{}", cmd)?;
        }
        self.stdin.flush()?;
        // TODO: return result
    }

    fn breakpoint_set(&mut self, addr: usize) -> Result<BpIdent> {
        self.send_cmd(&format!("bp {:#x}", addr))?;
        // TODO: parse and return breakpoint id
        String::new()
    }

    fn breakpoint_disable(&mut self, bp: BpIdent) -> Result<()> {
        self.send_cmd(&format!("bd {}", bp))
    }

    fn breakpoint_enable(&mut self, bp: BpIdent) -> Result<()> {
        self.send_cmd(&format!("be {}", bp))
    }

    fn breapoint_delete(&mut self, bp: BpIdent) -> Result<()> {
        self.send_cmd(&format!("bc {}", bp))
    }

    fn continue(&mut self) -> Result<String> {
        self.send_cmd("g")
    }
}

impl Drop for Debugger {
    fn drop(&mut self) {
        let _ = self.stdin.write_all(b"q\n");
    }
}
