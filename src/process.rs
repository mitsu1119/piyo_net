use procfs::process::all_processes;

pub fn search_processes(name: &str) -> Vec<i32> {
    let mut res: Vec<i32> = vec![];
    for ps in all_processes().unwrap() {
        if let Ok(stat) = ps.unwrap().stat() {
            if stat.comm == name {
                res.push(stat.pid);
            }
        }
    }

    res
}
