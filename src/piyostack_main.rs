use piyo_net::process::search_processes;

fn main() {
    let ps = search_processes("vim");
    println!("{:?}", ps);
}
