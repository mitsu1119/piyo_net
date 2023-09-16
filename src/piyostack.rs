use piyo_net::process::search_processes;

// piyonet のプロトコルスタック
#[derive(Debug)]
pub struct PiyoStack {}

impl PiyoStack {
    pub fn new() -> PiyoStack {
        PiyoStack {}
    }

    pub fn connect(&self) -> Result<(), i32> {
        let Some(nics) = search_processes("piyo_nic") else { return Err(2); };
        println!("{:?}", nics);

        Ok(())
    }
}
