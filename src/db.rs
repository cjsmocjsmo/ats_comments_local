use log::info;
use rusqlite::{Connection, Result};
use std::env;

pub fn create_tables() {
    let _at = create_acct_table();
    let _aut = create_auth_table();
    let _cmt = create_comments_table();
    let _et = create_estimates_table();
}

pub fn create_acct_table() -> Result<()> {
    let db_path = env::var("COMSERV_ACCT_DB").expect("COMSERV_ACCT_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            acctid TEXT NOT NULL,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            date TEXT NOT NULL
         )",
        (),
    )?;
    info!("Created accounts table: {:?}", &db_path);

    Ok(())
}

pub fn create_auth_table() -> Result<()> {
    let db_path = env::var("COMSERV_AUTH_DB").expect("COMSERV_AUTH_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS auth (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            acctid TEXT NOT NULL,
            token TEXT NOT NULL,
            indate TEXT NOT NULL,
            outdate TEXT NOT NULL
         )",
        (),
    )?;
    info!("Created auth table: {:?}", &db_path);

    Ok(())
}

pub fn create_comments_table() -> Result<()> {
    let db_path = env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS comments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            acctid TEXT NOT NULL,
            comid TEXT NOT NULL,
            name TEXT NOT NULL,
            email TEXT NOT NULL,
            comment TEXT NOT NULL,
            rating TEXT NOT NULL,
            date TEXT NOT NULL,
            accepted TEXT NOT NULL,
            rejected TEXT NOT NULL,
            mediapath TEXT NOT NULL
         )",
        (),
    )?;
    info!("Created comments table: {:?}", &db_path);

    Ok(())
}

pub fn create_estimates_table() -> Result<()> {
    let db_path = env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
    let conn = Connection::open(db_path.clone())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS estimates (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            acctid TEXT NOT NULL,
            estid TEXT NOT NULL,
            name TEXT NOT NULL,
            address TEXT NOT NULL,
            city TEXT NOT NULL,
            phone TEXT NOT NULL,
            email TEXT NOT NULL,
            comment TEXT NOT NULL,
            intake TEXT NOT NULL,
            reqdate TEXT NOT NULL,
            completed TEXT NOT NULL,
            mediapath TEXT NOT NULL
         )",
        (),
    )?;
    info!("Created estimates table: {:?}", &db_path);

    Ok(())
}
