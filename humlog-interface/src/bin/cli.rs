use humlog_core::Command;
use humlog_interface::get_command;
use std::io::{self, Write};
fn main() {
    let command = get_command();
    let stdout = io::stdout();
    let lock = stdout.lock();
    let mut w = io::BufWriter::with_capacity(8 * 1024, lock);
    let op = Command { command }.run();
    for j in op.iter() {
        write!(&mut w, "{}", j).unwrap();
        // print!("{}", j);
    }
}
