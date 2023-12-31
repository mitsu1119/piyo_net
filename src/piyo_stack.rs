use std::error;
use std::fmt;
use crate::process::search_processes;

use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;

type Result<T> = std::result::Result<T, PiyoStackError>;

// PiyoStack のエラー型
#[derive(Debug, Clone)]
pub struct PiyoStackError;

impl fmt::Display for PiyoStackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PiyoStackError")
    }
}

impl error::Error for PiyoStackError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

// piyonet のプロトコルスタック
#[derive(Debug)]
pub struct PiyoStack {}

impl PiyoStack {
    pub fn new() -> PiyoStack {
        PiyoStack {}
    }

    pub fn connect(&self) -> Result<()> {
        let Some(nics) = search_processes("piyo_nic") else { return Err(PiyoStackError); };

        let pid = Pid::from_raw(nics[0]);
        kill(pid, Signal::SIGUSR1);

        Ok(())
    }
}
