use crate::types;
use log::info;
use md5::{Digest, Md5};
use rusqlite::Connection;
use std::env;

pub fn create_hash(email: String) -> String {
    let mut hasher = Md5::new();
    hasher.update(email);
    let result = hasher.finalize();
    let acctid = format!("{:x}", result);

    acctid
}

pub fn has_account(email: String) -> bool {
    let acctid = create_hash(email.clone());
    let db_path = env::var("COMSERV_ACCT_DB").expect("COMSERV_ACCT_DB not set");
    let conn = Connection::open(db_path.clone()).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM accounts WHERE acctid = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&acctid]).unwrap();
    let mut result = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let acct = types::Account {
            acctid: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            email: row.get(3).unwrap(),
            date: row.get(4).unwrap(),
        };
        info!("Account: {:?}", acct);
        result.push(acct);
    }

    if result.len() > 0 {
        info!("account found");
        return true
    } else {
        info!("Account not found");
        return false
    }
}

pub fn create_account(namez: String, emailz: String) -> types::Account {
    let acct_id = create_hash(emailz.clone());
    let acct = types::Account {
        acctid: acct_id,
        name: namez,
        email: emailz,
        date: chrono::Local::now().naive_local().to_string(),
    };
    let db_path = env::var("COMSERV_ACCT_DB").expect("COMSERV_ACCT_DB not set");
    let conn = Connection::open(db_path.clone()).unwrap();
    let mut stmt = conn
        .prepare("INSERT INTO accounts (acctid, name, email, date) VALUES (?1, ?2, ?3, ?4)")
        .unwrap();
    let _result = stmt
        .execute(&[&acct.acctid, &acct.name, &acct.email, &acct.date])
        .unwrap();

    acct
}

pub fn account_info_from_email(email: String) -> Vec<types::Account> {
    let acctid = create_hash(email.clone());
    let db_path = env::var("COMSERV_ACCT_DB").expect("COMSERV_ACCT_DB not set");
    let conn = Connection::open(db_path.clone()).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM accounts WHERE acctid = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&acctid]).unwrap();
    let mut result = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let acct = types::Account {
            acctid: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            email: row.get(3).unwrap(),
            date: row.get(4).unwrap(),
        };
        info!("Account: {:?}", acct);
        result.push(acct);
    }

    result
}

pub fn account_info_from_acctid(acctid: String) -> Vec<types::Account> {
    let db_path = env::var("COMSERV_ACCT_DB").expect("COMSERV_ACCT_DB not set");
    let conn = Connection::open(db_path.clone()).unwrap();
    let mut stmt = conn
        .prepare("SELECT * FROM accounts WHERE acctid = ?1")
        .unwrap();
    let mut rows = stmt.query(&[&acctid]).unwrap();
    let mut result = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let acct = types::Account {
            acctid: row.get(1).unwrap(),
            name: row.get(2).unwrap(),
            email: row.get(3).unwrap(),
            date: row.get(4).unwrap(),
        };
        info!("Account: {:?}", acct);
        result.push(acct);
    }

    result
}