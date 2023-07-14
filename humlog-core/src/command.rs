use std::io::{BufRead, BufReader};
use std::process::Command as StdCommand;
use std::process::Stdio;
use std::sync::mpsc::{self, Iter, Receiver};
use std::thread;
pub struct Command {
    pub command: String,
}

impl Command {
    pub fn run(&self) -> CommandOutput<String> {
        let (tx, rx) = mpsc::channel();
        let command = self.command.clone();
        thread::spawn(move || {
            let mut child_process = StdCommand::new(command)
                .stdout(Stdio::piped())
                .spawn()
                .expect("Cannot spawn the command");
            let stdout = child_process
                .stdout
                .as_mut()
                .expect("Cannot receive stdout");
            let mut br = BufReader::new(stdout);
            let mut s = String::new();
            loop {
                match br.read_line(&mut s) {
                    Ok(0) => break,
                    Ok(_) => tx
                        .send(s.clone())
                        .expect("Cannot communicate with the receiver"),
                    Err(_) => panic!("Error in transmitting"),
                }
                s.clear();
            }
        });
        CommandOutput { output: rx }
    }
}

pub struct CommandOutput<T> {
    output: Receiver<T>,
}

impl<T> CommandOutput<T> {
    pub fn iter<'a>(&'a self) -> COIter<'a, T> {
        COIter(self.output.iter())
    }
}

pub struct COIter<'a, T>(Iter<'a, T>);
impl<'a, T> Iterator for COIter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
