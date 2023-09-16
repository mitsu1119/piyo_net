mod piyostack;
use piyostack::PiyoStack;

fn main() {
    let stack = PiyoStack::new();
    println!("{:?}", stack);

    stack.connect().expect("Failed to connect to NIC");
}
