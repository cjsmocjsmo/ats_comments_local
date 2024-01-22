use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub acctid: String,
    pub name: String,
    pub email: String,
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Auth {
    pub acctid: String,
    pub token: String,
    pub indate: String,
    pub outdate: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormData {
    pub name: String,
    pub email: String,
    pub rating: String,
    pub comment: String,
    pub filepicker: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullComment {
    pub acctid: String,
    pub comid: String,
    pub name: String,
    pub email: String,
    pub comment: String,
    pub rating: String,
    pub date: String,
    pub accepted: String,
    pub rejected: String,
    pub mediapath: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Estimate {
    pub acctid: String,
    pub estid: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub comment: String,
    pub intake: String,
    pub reqdate: String,
    pub completed: String,
    pub mediapath: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MailEstimate {
    pub estid: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub phone: String,
    pub email: String,
    pub comment: String,
    pub intake: String,
    pub reqdate: String,
}