use crate::accounts;
use crate::sendmail;
use crate::types;
use actix_multipart::Multipart;
use actix_web::{get, post, web, web::Redirect, Error, HttpResponse, Responder};

use chrono::prelude::*;
use file_owner::PathExt;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures::{StreamExt, TryStreamExt};
use log::info;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;
use sanitize_filename;


#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("\nATS Comments Server up and running!\n")
}

#[get("/allcom")]
pub async fn all_comments() -> impl Responder {
    let com_serv_comments_db =
        env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
    let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM comments").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut comment_vec = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let acctidz: String = row.get(1).unwrap();
        let comidz: String = row.get(2).unwrap();
        let namez: String = row.get(3).unwrap();
        let emailz: String = row.get(4).unwrap();
        let commentz: String = row.get(5).unwrap();
        let ratingz: String = row.get(6).unwrap();
        let datez: String = row.get(7).unwrap();
        let acceptedz: String = row.get(8).unwrap();
        let rejectedz: String = row.get(9).unwrap();

        let comment = types::FullComment {
            acctid: acctidz.to_string(),
            comid: comidz.to_string(),
            name: namez.to_string(),
            email: emailz.to_string(),
            comment: commentz.to_string(),
            rating: ratingz.to_string(),
            date: datez.to_string(),
            accepted: acceptedz.to_string(),
            rejected: rejectedz.to_string(),
        };
        info!("Comment: {:?}", comment);
        comment_vec.push(comment);
    }

    HttpResponse::Ok().json(comment_vec)
}

#[get("/addcom/{name}/{email}/{comment}/{rating}")]
pub async fn add_comment(f: web::Path<(String, String, String, String)>) -> impl Responder {
    let (name, email, comment, rating) = f.into_inner();
    let comidz = accounts::create_hash(comment.clone());
    info!("name: {:#?}", name);
    info!("email: {:#?}", email);
    info!("comment: {:#?}", comment);
    info!("rating: {:#?}", rating);
    let has_acct = accounts::has_account(email.clone());
    if has_acct {
        let acct_info = accounts::account_info_from_email(email.clone());
        let acctid = &acct_info[0].acctid;
        let datae = &acct_info[0].date;
        let commet = types::FullComment {
            acctid: acctid.to_string(),
            comid: comidz.clone(),
            name: name.clone(),
            email: email.clone(),
            comment: comment.clone(),
            rating: rating.clone(),
            date: datae.to_string(),
            accepted: "None".to_string(),
            rejected: "None".to_string(),
        };
        info!("has_account Comment: {:#?}", commet);
        let com_serv_comments_db =
            env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
        let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
        conn.execute(
            "INSERT INTO comments (acctid, comid, name, email, comment, date, accepted, rejected) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            &[&commet.acctid, &commet.comid, &commet.name, &commet.email, &commet.comment, &commet.date, &commet.accepted, &commet.rejected],
        )
        .expect("unable to insert comment");

        let mailz = sendmail::send_com_mail(commet).await;
        match mailz {
            Ok(_) => info!("Mail Sent"),
            Err(e) => info!("Mail Error: {:?}", e),
        };
    } else {
        let acct_info = accounts::create_account(name.clone(), email.clone());
        let acctid = &acct_info.acctid;
        let datae = &acct_info.date;
        let fullcomment = types::FullComment {
            acctid: acctid.to_string(),
            comid: comidz.clone(),
            name: name.clone(),
            email: email.clone(),
            comment: comment.clone(),
            rating: rating.clone(),
            date: datae.to_string(),
            accepted: "None".to_string(),
            rejected: "None".to_string(),
        };
        info!("create_account Comment: {:#?}", fullcomment);
        let com_serv_comments_db =
            env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
        let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
        conn.execute(
            "INSERT INTO comments (acctid, comid, name, email, comment, rating, date, accepted, rejected) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            &[&fullcomment.acctid, &fullcomment.comid, &fullcomment.name, &fullcomment.email, &fullcomment.comment, &rating, &fullcomment.date, &fullcomment.accepted, &fullcomment.rejected],
        )
        .expect("unable to insert comment");

        let mailz = sendmail::send_com_mail(fullcomment).await;
        match mailz {
            Ok(_) => info!("Mail Sent"),
            Err(e) => info!("Mail Error: {:?}", e),
        };
    };

    HttpResponse::Ok().body("Comment inserted into db\n")
}

#[get("/allesti")]
pub async fn all_estimates() -> impl Responder {
    let com_serv_estimates_db =
        env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
    let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
    let mut stmt = conn.prepare("SELECT * FROM estimates").unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut estimate_vec = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let estimate = types::Estimate {
            acctid: row.get(1).unwrap(),
            estid: row.get(2).unwrap(),
            name: row.get(3).unwrap(),
            address: row.get(4).unwrap(),
            city: row.get(5).unwrap(),
            phone: row.get(6).unwrap(),
            email: row.get(7).unwrap(),
            comment: row.get(8).unwrap(),
            intake: row.get(9).unwrap(),
            reqdate: row.get(10).unwrap(),
            completed: row.get(11).unwrap(),
        };
        info!("Estimate: {:?}", estimate);
        estimate_vec.push(estimate);
    }

    HttpResponse::Ok().json(estimate_vec)
}

#[get("/addesti/{name}/{address}/{city}/{phone}/{email}/{comment}/{reqdate}")]
pub async fn add_estimate(
    f: web::Path<(String, String, String, String, String, String, String)>,
) -> impl Responder {
    let (name, address, city, phone, email, comment, reqdate) = f.into_inner();
    let eid = name.clone() + &address + &city + &phone + &email;
    let estidz = accounts::create_hash(eid.clone());
    let has_acct = accounts::has_account(email.clone());
    if has_acct {
        let acct_info = accounts::account_info_from_email(email.clone());
        let acctid = &acct_info[0].acctid;
        let today = Local::now().format("%Y-%m-%d").to_string();
        let estimate = types::Estimate {
            acctid: acctid.to_string(),
            estid: estidz.clone(),
            name: name.clone(),
            address: address.clone(),
            city: city.clone(),
            phone: phone.clone(),
            email: email.clone(),
            comment: comment.clone(),
            intake: today.clone(),
            reqdate: reqdate.clone(),
            completed: "No".to_string(),
        };
        info!("has_account Estimate: {:#?}", estimate);
        let com_serv_estimates_db =
            env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
        let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
        conn.execute(
            "INSERT INTO estimates (acctid, estid, name, address, city, phone, email, comment, intake, reqdate, completed) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            &[&estimate.acctid, &estimate.estid, &estimate.name, &estimate.address, &estimate.city, &estimate.phone, &estimate.email, &estimate.comment, &estimate.intake, &estimate.reqdate, &estimate.completed],
        )
        .expect("unable to insert estimate");

        let mailz = sendmail::send_esti_mail(estimate).await;
        match mailz {
            Ok(_) => info!("Esti Mail Sent"),
            Err(e) => info!("Mail Error: {:?}", e),
        };
    } else {
        let acct_info = accounts::create_account(name.clone(), email.clone());
        let acctid = &acct_info.acctid;
        let today = Local::now().format("%Y-%m-%d").to_string();
        let estimate = types::Estimate {
            acctid: acctid.to_string(),
            estid: estidz.clone(),
            name: name.clone(),
            address: address.clone(),
            city: city.clone(),
            phone: phone.clone(),
            email: email.clone(),
            comment: comment.clone(),
            intake: today.clone(),
            reqdate: reqdate.clone(),
            completed: "No".to_string(),
        };
        info!("create_account Estimate: {:#?}", estimate);
        let com_serv_estimates_db =
            env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
        let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
        conn.execute(
            "INSERT INTO estimates (acctid, estid, name, address, city, phone, email, comment, intake, reqdate, completed) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            &[&estimate.acctid, &estimate.estid, &estimate.name, &estimate.address, &estimate.city, &estimate.phone, &estimate.email, &estimate.comment, &estimate.intake, &estimate.reqdate, &estimate.completed],
        ).expect("unable to insert estimate");

        let mailz = sendmail::send_esti_mail(estimate).await;
        match mailz {
            Ok(_) => info!("Esti Mail Sent"),
            Err(e) => info!("Mail Error: {:?}", e),
        };
    };

    HttpResponse::Ok().body("\nEstimate inserted into db\n")
}

#[get("/accept/{msgid}")]
pub async fn accept_comment(id: web::Path<String>) -> impl Responder {
    let msgid = id.into_inner();
    let todays_date = Local::now().format("%Y-%m-%d").to_string();
    info!("msgid: {:#?}", msgid);
    info!("todays_date: {:#?}", todays_date);
    let com_serv_comments_db =
        env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
    let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
    conn.execute(
        "UPDATE comments SET accepted = ?1 WHERE comid = ?2",
        &[&todays_date, &msgid],
    )
    .expect("unable to update comment");

    Redirect::to("https://alpha-tree.biz/accepted").permanent();

    HttpResponse::Ok().body("\nComment Accepted\n")
}

#[get("/reject/{msgid}")]
pub async fn reject_comment(id: web::Path<String>) -> impl Responder {
    let msgid = id.into_inner();
    let todays_date = Local::now().format("%Y-%m-%d").to_string();
    info!("msgid: {:#?}", msgid);
    info!("todays_date: {:#?}", todays_date);
    let com_serv_comments_db =
        env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
    let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
    conn.execute(
        "UPDATE comments SET rejected = ?1 WHERE comid = ?2",
        &[&todays_date, &msgid],
    )
    .expect("unable to update comment");

    Redirect::to("https://alpha-tree.biz/rejected").permanent();

    HttpResponse::Ok().body("\nComment Rejected\n")
}

#[get("/completed/{msgid}")]
pub async fn esti_complete(id: web::Path<String>) -> impl Responder {
    let msgid = id.into_inner();
    let todays_date = Local::now().format("%Y-%m-%d").to_string();
    let com_serv_estimates_db =
        env::var("COMSERV_ESTIMATES_DB").expect("COMSERV_ESTIMATES_DB not set");
    let conn = rusqlite::Connection::open(com_serv_estimates_db).unwrap();
    conn.execute(
        "UPDATE estimates SET completed = ?1 WHERE estid = ?2",
        &[&todays_date, &msgid],
    )
    .expect("unable to update estimate");

    Redirect::to("https://alpha-tree.biz/completed").permanent();

    HttpResponse::Ok().body("\nEstimate Completed\n")
}

fn create_backup(dbpath: String, f_name: String) -> Result<(), std::io::Error> {
    let tar_gz = File::create(f_name)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all("backup/", dbpath)?;
    Ok(())
}

fn my_fix_perms(f_name: String) -> std::io::Result<()> {
    let mut perms = fs::metadata(f_name.clone())?.permissions();
    perms.set_readonly(true);
    fs::set_permissions(f_name.clone(), perms)?;
    Ok(())
}

fn my_fix_owners(f_name: String) -> std::io::Result<()> {
    f_name.set_owner("root").unwrap();
    f_name.set_group("root").unwrap();
    Ok(())
}

#[get("/backup")]
pub async fn backup() -> impl Responder {
    info!("Starting Backup");
    let up_loads_dir = env::var("COMSERV_UPLOADS").expect("COMSERV_UPLOADS not set");
    let current_date = Local::now().date_naive();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();
    let archive_name = format!("{}{}{}archive.tar.gz", year, month, day);
    let fname = format!("{}/{}", up_loads_dir, archive_name);
    info!("fname: {:#?}", fname);

    let db_path = env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
    let _bup = match create_backup(db_path, fname.clone()) {
        Ok(_) => info!("Backup Created"),
        Err(e) => info!("Backup Error: {:?}", e),
    };

    let _fperms = match my_fix_perms(fname.clone()) {
        Ok(_) => info!("Permissions Fixed"),
        Err(e) => info!("Permissions Error: {:?}", e),
    };

    let _fowners = match my_fix_owners(fname.clone()) {
        Ok(_) => info!("Owners Fixed"),
        Err(e) => info!("Owners Error: {:?}", e),
    };
    let msg = "use this command\nwget https://alpha-tree.biz/uploads/".to_string() + &archive_name;

    HttpResponse::Ok().body(msg)
}

// #[post("/upload")]
// pub async fn upload_file(form: web::Form<types::FormData>) -> impl Responder {
//     let name = &form.name;
//     let email = &form.email;
//     let rating = &form.rating;
//     let comment = &form.comment;
//     let filepicker = &form.filepicker;
//     info!("name: {:#?}", name);
//     info!("email: {:#?}", email);
//     info!("rating: {:#?}", rating);
//     info!("comment: {:#?}", comment);
//     info!("filepicker: {:#?}", filepicker);

//     //filpicer is an image that needs to be saved to disk
//     let file_path = env::var("COMSERV_UPLOADS").unwrap();
//     let file_name = format!("{}/{}", file_path, filepicker);
//     let file_data = BASE64_STANDARD.decode(filepicker).unwrap();
//     let mut file = File::create(file_name).unwrap();
//     file.write_all(&file_data).unwrap();

//     let has_acct = accounts::has_account(email.clone());
//     if has_acct {
//         let acct_info = accounts::account_info_from_email(email.clone());
//         let acctid = &acct_info[0].acctid;
//         let datae = &acct_info[0].date;
//         let commet = types::FullComment {
//             acctid: acctid.to_string(),
//             comid: "None".to_string(),
//             name: name.clone(),
//             email: email.clone(),
//             comment: comment.clone(),
//             rating: rating.clone(),
//             date: datae.to_string(),
//             accepted: "None".to_string(),
//             rejected: "None".to_string(),
//         };
//         info!("has_account Comment: {:#?}", commet);
//         let com_serv_comments_db =
//             env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
//         let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
//         conn.execute(
//             "INSERT INTO comments (acctid, comid, name, email, comment, date, accepted, rejected) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
//             &[&commet.acctid, &commet.comid, &commet.name, &commet.email, &commet.comment, &commet.date, &commet.accepted, &commet.rejected],
//         )
//         .expect("unable to insert comment");

//         let mailz = sendmail::send_com_mail(commet).await;
//         match mailz {
//             Ok(_) => info!("Mail Sent"),
//             Err(e) => info!("Mail Error: {:?}", e),
//         };
//     } else {
//         let acct_info = accounts::create_account(name.clone(), email.clone());
//         let acctid = &acct_info.acctid;
//         let datae = &acct_info.date;
//         let fullcomment = types::FullComment {
//             acctid: acctid.to_string(),
//             comid: "None".to_string(),
//             name: name.clone(),
//             email: email.clone(),
//             comment: comment.clone(),
//             rating: rating.clone(),
//             date: datae.to_string(),
//             accepted: "None".to_string(),
//             rejected: "None".to_string(),
//         };
//         info!("create_account Comment: {:#?}", fullcomment);
//         let com_serv_comments_db =
//             env::var("COMSERV_COMMENTS_DB").expect("COMSERV_COMMENTS_DB not set");
//         let conn = rusqlite::Connection::open(com_serv_comments_db).unwrap();
//         conn.execute(
//             "INSERT INTO comments (acctid, comid, name, email, comment, rating, date, accepted, rejected) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
//             &[&fullcomment.acctid, &fullcomment.comid, &fullcomment.name, &fullcomment.email, &fullcomment.comment, &rating, &fullcomment.date, &fullcomment.accepted, &fullcomment.rejected],
//         )
//         .expect("unable to insert comment");

//         let mailz = sendmail::send_com_mail(fullcomment).await;
//         match mailz {
//             Ok(_) => info!("Mail Sent"),
//             Err(e) => info!("Mail Error: {:?}", e),
//         };
//     };

//     HttpResponse::Ok().body("File Saved\n")
// }

#[get("/backup")]
pub async fn save_file(mut payload: Multipart, file_path: String) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        let _content_type = field.content_disposition();
        //let filename = content_type.get_filename().unwrap();
        let filepath = format!(".{}", file_path);

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap()
            .expect("unable to create file");

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap()
                .expect("unable to write file");
        }
    }

    Ok(HttpResponse::Ok().body("File Saved\n"))
}


#[post("/upload")]
async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut name = String::new();
    let mut email = String::new();
    let mut rating = String::new();
    let mut image_path = String::new();
    let uu_id = Uuid::new_v4();

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition();
        let name_content = content_disposition.get_name().unwrap();

        match name_content {
            "filepicker" => {
                let filename = format!("{}-{}", uu_id, content_disposition.get_filename().unwrap());
                image_path = format!("./{}", sanitize_filename::sanitize(&filename.clone()));
                let filepath = image_path.clone();
                let mut f = web::block(move || std::fs::File::create(filepath.clone())).await.unwrap().expect("unable to create file");

                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f)).await??;
                }
                
            },
            "name" => {
                while let Some(chunk) = field.next().await {
                    let namez = chunk.unwrap();
                    name = String::from_utf8(namez.to_vec()).unwrap();
                }
            },
            "email" => {
                while let Some(chunk) = field.next().await {
                    let emailz = chunk.unwrap();
                    email = String::from_utf8(emailz.to_vec()).unwrap();
                }
            },
            "rating" => {
                while let Some(chunk) = field.next().await {
                    let ratingz = chunk.unwrap();
                    rating = String::from_utf8(ratingz.to_vec()).unwrap();
                }
            },
            _ => {}
        }
    }

    info!("name: {:#?}", name);
    info!("email: {:#?}", email);
    info!("rating: {:#?}", rating);
    info!("image_path: {:#?}", image_path);
    // Write name, email, rating to the database
    // This is pseudocode, replace with your actual database code
    // db.insert("comments", vec![("name", name), ("email", email), ("rating", rating)]);

    Ok(HttpResponse::Ok().into())
}