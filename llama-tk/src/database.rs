use std::path::PathBuf;

use rusqlite::Connection;

pub fn setup(path: &PathBuf) {
    let db_path = path.join("llama-db.sqlite");
    let db = Connection::open(db_path).unwrap();

    db.execute(
        "CREATE TABLE IF NOT EXISTS files (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            path TEXT NOT NULL
        )",
        (),
    )
    .unwrap();

    // List out all tables
    let mut stmt = db
        .prepare("SELECT name FROM sqlite_master WHERE type='table'")
        .unwrap();

    let mut rows = stmt.query([]).unwrap();

    let mut names: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        names.push(row.get(0).unwrap());
    }

    for table in names {
        println!("{}", table);
    }
}
