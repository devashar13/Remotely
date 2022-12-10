use clap::Parser;
use rusqlite::{Connection, Result};

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

pub fn add_ssh(
    ip: &String,
    name: &String,
    user: &String,
) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("pulao.db")?;
    conn.execute(
        "INSERT INTO pulao (ip,name,user) values (?1,?2,?3)",
        [ip, name, user],
    )?;

    Ok(())
}
