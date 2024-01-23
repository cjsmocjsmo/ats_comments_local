use actix_cors::Cors;
use actix_files as fs;
use actix_web::{App, HttpServer};
use env_logger;
use log::info;
// use openssl::{
//     pkey::{PKey, Private},
//     ssl::{SslAcceptor, SslMethod},
// };
use std::env;
// use std::fs::File;
// use std::io::Read;

pub mod accounts;
pub mod db;
pub mod functions;
pub mod sendmail;
pub mod server;
pub mod types;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("/usr/share/ats_comments/ats_comments_local/atscomments.env").ok();

    env_logger::init();

    let db_check = functions::db_file_checks();
    info!("db_check result: {:?}", db_check);
    if db_check == 6 {
        let _create_tables = db::create_tables();
    }

    let uploads_path = env::var("COMSERV_UPLOADS").unwrap();

    // let key_file_path = env::var("COMSERV_KEY_PEM").unwrap();
    // let cert_file_path = env::var("COMSERV_CERT_PEM").unwrap();

    // let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    // builder
    //     .set_private_key(&load_encrypted_private_key(key_file_path))
    //     .unwrap();

    // builder.set_certificate_chain_file(cert_file_path).unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(server::test)
            .service(server::add_comment)
            .service(server::all_comments)
            .service(server::add_estimate)
            .service(server::all_estimates)
            .service(server::accept_comment)
            .service(server::reject_comment)
            .service(server::esti_complete)
            .service(server::backup_file)
            .service(fs::Files::new("/uploads", uploads_path.clone()).show_files_listing())
    })
    // .bind_openssl("0.0.0.0:443", builder)?
    .bind("192.168.0.91:8080")?
    .run()
    .await
}

// #[post("/upload")]
// async fn upload_file(form: web::Form<FormData>) -> Result<HttpResponse, Error> {
//     while let Some(item) = payload.next().await {
//         let mut field = item?;
//         let content_type = field.content_disposition().unwrap();
//         let filename = content_type.get_filename().unwrap();
//         let filepath = format!("/uploads/{}", sanitize_filename::sanitize(&filename));

//         let mut f = web::block(|| std::fs::File::create(filepath))
//             .await
//             .unwrap();

//         while let Some(chunk) = field.next().await {
//             let data = chunk.unwrap();
//             f = web::block(move || f.write_all(&data).map(|_| f)).await?;
//         }
//     }

//     Ok(HttpResponse::Ok().into())
// }


// fn load_encrypted_private_key(key_path: String) -> PKey<Private> {
//     let mut file = File::open(key_path).unwrap();
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer).expect("Failed to read file");

//     PKey::private_key_from_pem_passphrase(&buffer, b"password").unwrap()
// }
