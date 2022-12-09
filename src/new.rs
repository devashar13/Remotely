use clap::Parser;
// use rusqlite::Connection;

/// Simple program to subtract numbers
#[derive(Parser, Debug)]
struct Args {
    //    keeping it string for now
    #[clap(short = 'i', long)]
    ip: String,
    /// Second number to add
    #[clap(short = 'n', long)]
    name: String,

    #[clap(short = 'u', long)]
    user: String,
}

pub fn add_ssh(ip: &String, name: &String, user: &String) {
    let conn = Connection::open("pulao.db");
    
}
