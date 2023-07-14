use clap::{Command,Arg};

pub fn get_command()->String{
    let matches = Command::new(std::env!("CARGO_PKG_NAME")).arg(Arg::new("command").help("comand string")).get_matches();
    matches.get_one::<String>("command").expect("command not provided").clone()
}