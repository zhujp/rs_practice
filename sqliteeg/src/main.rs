use rusqlite::{Connection, Result};
fn main()->Result<()> {
    let conn = Connection::open("service.db")?;
    // create_table(&conn)?;
    // insert_user(&conn,"John", 30)?;
    // insert_user(&conn,"vilay", 33)?;
    let users = query_users(&conn)?;
    println!("{:?}",users);
    // update_user(&conn,1, 33)?;
    delete_user(&conn,1)?;
    Ok(())
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             age INTEGER NOT NULL
         )",
        (), // No parameters
    )?;

    Ok(())
}


fn insert_user(conn: &Connection, name: &str, age: i32) -> Result<()> {
    conn.execute(
        "INSERT INTO users (name, age) VALUES (?1, ?2)",
        (name, age),
    )?;

    Ok(())
}

fn query_users(conn: &Connection) -> Result<Vec<(i32, String, i32)>> {
    let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;
    let users_iter = stmt.query_map((), |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, i32>(2)?))
    });
    let x = users_iter?.collect::<Result<Vec<_>, _>>();
    x
}

fn update_user(conn: &Connection, id: i32, new_age: i32) -> Result<()> {
    conn.execute(
        "UPDATE users SET age = ?1 WHERE id = ?2",
        (new_age, id),
    )?;

    Ok(())
}

fn delete_user(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM users WHERE id = ?1", (id,))?;

    Ok(())
}
