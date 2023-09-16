use procfs::process::all_processes;

pub fn search_processes(name: &str) -> Option<Vec<i32>> {
    let mut res: Vec<i32> = vec![];
    for ps in all_processes().unwrap() {
        if let Ok(stat) = ps.unwrap().stat() {
            if stat.comm == name {
                res.push(stat.pid);
            }
        }
    }

    if res.len() == 0 {
        None
    } else {
        Some(res)
    }
}
