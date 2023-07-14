use std::io::{BufRead, BufReader};
use std::process::Command as StdCommand;
use std::process::Stdio;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
pub struct Command {
    pub command: String,
}

impl Command {
    pub fn run(&self) -> CommandOutput<String, ()> {
        let buffer = Arc::new(Mutex::new(vec![]));
        let command = self.command.clone();
        let thread_buffer = Arc::clone(&buffer);
        let handle = thread::spawn(move || {
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
                    Ok(_) => {
                        let mut buffer = thread_buffer.lock().unwrap();
                        buffer.push(s.clone());
                    }
                    Err(_) => panic!("Error in transmitting"),
                }
                s.clear();
            }
        });
        CommandOutput {
            output: buffer,
            handle,
        }
    }
}

pub struct CommandOutput<T, U> {
    pub output: Arc<Mutex<Vec<T>>>,
    pub handle: JoinHandle<U>,
}

// impl<T> CommandOutput<T> {
//     pub fn iter<'a>(&'a self) -> COIter<'a, T> {
//         COIter(self.output.lock().unwrap().iter().collect())
//     }
// }

// pub struct COIter<'a, T>(Iter<'a, T>);
// impl<'a, T> Iterator for COIter<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.next()
//     }
// }
