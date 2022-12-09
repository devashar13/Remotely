use rusqlite::{Connection, Result};

pub fn connect_db() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("pulao.db")?;
    conn.execute(
        "create table if not exists pulao (
      id integer primary key,
      ip text not null unique,
      name text not null unique,
      user text not null unique    
    )",
        [],
    )?;

    Ok(())
}
