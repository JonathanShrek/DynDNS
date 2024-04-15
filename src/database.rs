pub mod database_functions {

  use rusqlite::{
      Connection,
      Result,
      Error
    };

  pub fn create_ip_table() {
    // open a connection to the sqlite db or create a new one if it doesn't exist
    let conn = Connection::open("dyndns.db").expect("Failed to open or create a connection with the dyndns.db");

    // execute sql statements to create a table named "users"
    conn.execute(
      "CREATE TABLE IF NOT EXISTS ip_addresses (
        id INTEGER PRIMARY KEY,
        ip TEXT NOT NULL)",
        [],
    ).expect("Failed to create the database");
  }

  pub fn insert_or_update(ip: &str) {
    // open a connection to the sqlite db or create a new one if it doesn't exist
    let conn = Connection::open("dyndns.db").expect("Failed to open or create a connection with the dyndns.db");

    // insert data into the ip_addresses table
    conn.execute(
      "INSERT OR REPLACE INTO ip_addresses (ip) VALUES (?1)",
      [ip],
    ).expect("Failed to insert into the table");
  }

  pub fn get_all_ip_addresses() {
    // open a connection to the sqlite db or create a new one if it doesn't exist
    let conn = Connection::open("dyndns.db").expect("Failed to open or create a connection with the dyndns.db");

    // query all ip addresses from the "ip" table
    let mut stmt = conn.prepare("SELECT id, ip FROM ip_addresses").expect("Failed to select all from the table");
    let ip_iter = stmt.query_map([], |row| {
      Ok((
        row.get::<usize, i32>(0)?, // id
        row.get::<usize, String>(1)?, // ip address
      ))
    }).expect("Failed to parse select statement into iterable object");

    for ip in ip_iter {
      let (id, ip) = ip.expect("Failed to assign iteral ip object into variables");
      println!("ID: {}, IP: {}", id, ip);
    }
  }

  pub fn get_latest_ip() -> Result<String, Error> {
    // open a connection to the sqlite db or create a new one if it doesn't exist
    let conn = Connection::open("dyndns.db")?;

    // Execute a SQL query to select the row with the maximum ID
    let mut stmt = conn.prepare("SELECT ip FROM ip_addresses ORDER BY id DESC LIMIT 1")?;

    // Execute the query and return the value of the 'ip' column
    let ip_result: Result<String> = match stmt.query_row([], |row| {
        row.get(0)
    }) {
        Ok(ip) => Ok(ip),
        Err(e) => Err(e),
    };

    // Return the result of the query
    ip_result
  }

  pub fn drop_table(table: &str) {
    // open a connection to the sqlite db or create a new one if it doesn't exist
    let conn = Connection::open("dyndns.db").expect("Failed to open or create a connection with the dyndns.db");

    // Execute the DROP TABLE statement
    conn.execute(
        &format!("DROP TABLE IF EXISTS {}", table),
        [],
    ).expect("Failed to drop the table");
  }
}