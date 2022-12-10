use rusqlite::{Connection, Result};

use subprocess::Exec;

pub fn connect_db() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("pulao.db")?;
    conn.execute(
        "create table if not exists pulao (
      id integer primary key,
      ip text not null unique,
      name text not null unique,
      user text not null     
    )",
        [],
    )?;

    Ok(())
}

pub fn list() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("pulao.db")?;
    let mut dbs = conn.prepare("SELECT * FROM pulao")?;
    let mut rows = dbs.query(rusqlite::params![])?;
    while let Some(row) = rows.next()? {
        let ip: String = row.get(1)?;
        let name: String = row.get(2)?;
        let user: String = row.get(3)?;

        println!("{} {} {}", ip, name, user);
    }

    Ok(())
}

pub fn run(name: &String) -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("pulao.db")?;
    let query = "SELECT ip, user FROM pulao WHERE name = $1";
    let mut stmt = conn.prepare(query).unwrap();
    let mut result = stmt.query(&[&name]).unwrap();
    while let Some(row) = result.next()? {
        let ip: String = row.get(0)?;
        let user: String = row.get(1)?;

        let ssh_ip = format!("ssh {}@{}", &user, &ip);
        Exec::shell(ssh_ip).join()?;
    }

    // let mut dbs = conn.prepare("SELECT (ip,user) FROM pulao WHERE name = (?)")?;
    // dbs.execute(["nigga"])?;
    // let mut rows = dbs.query(rusqlite::params![])?;
    // while let Some(row) = rows.next()? {
    //     let ip: String = row.get(0)?;
    //     let user: String = row.get(1)?;
    //     let ssh_ip = user + "@" + &ip;
    //     let output = Command::new(&ssh_ip)
    //         .arg("ubuntu@192.168.2.1")
    //         .output()
    //         .expect("failed to execute ssh command");
    // }

    Ok(())
}
