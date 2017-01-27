use std::io::{Write, BufWriter, BufReader, BufRead, Result};
use std::process::{ChildStdin, ChildStdout};

pub struct Debugger {
    stdin: BufWriter<ChildStdin>,
    stdout: BufReader<ChildStdout>,
}

impl Debugger {
    pub fn send_cmd_raw(&mut self, cmd: &str) -> Result<()> {
        if cmd.ends_with("\n") {
            write!(self.stdin, "{}", cmd)?;
        } else {
            writeln!(self.stdin, "{}", cmd)?;
        }
        self.stdin.flush()?;
    }

    pub fn start(pid: u32) -> Result<Debugger> {
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
}

impl Drop for Debugger {
    fn drop(&mut self) {
        let _ = self.stdin.write_all(b"q\n");
    }
}
