// use log::info;
// use std::env;

// pub fn set_env_vars() {
//     let key_pem = env::var("COMSERV_KEY_PEM");
//     if key_pem.is_err() {
//         env::set_var(
//             "COMSERV_KEY_PEM",
//             "/etc/letsencrypt/archive/atstest.xyz/privkey1.pem",
//         );
//     };

//     let cert_pem = env::var("COMSERV_CERT_PEM");
//     if cert_pem.is_err() {
//         env::set_var(
//             "COMSERV_CERT_PEM",
//             "/etc/letsencrypt/archive/atstest.xyz/fullchain1.pem",
//         );
//     };
    
//     let comserv_db_path = env::var("COMSERV_DB_PATH");
//     if comserv_db_path.is_err() {
//         env::set_var("COMSERV_DB_PATH", "/usr/share/ats_comments/ats_comments/db");
//     };

//     let comserv_uploads = env::var("COMSERV_UPLOADS");
//     if comserv_uploads.is_err() {
//         env::set_var("COMSERV_UPLOADS", "/usr/share/ats_comments/uploads");
//     };

//     let comserv_acct_db = env::var("COMSERV_ACCT_DB");
//     if comserv_acct_db.is_err() {
//         env::set_var(
//             "COMSERV_ACCT_DB",
//             "/usr/share/ats_comments/ats_comments/db/accounts.db",
//         );
//     };

//     let comserv_auth_db = env::var("COMSERV_AUTH_DB");
//     if comserv_auth_db.is_err() {
//         env::set_var(
//             "COMSERV_AUTH_DB",
//             "/usr/share/ats_comments/ats_comments/db/auth.db",
//         );
//     };

//     let comserv_comments_db = env::var("COMSERV_COMMENTS_DB");
//     if comserv_comments_db.is_err() {
//         env::set_var(
//             "COMSERV_COMMENTS_DB",
//             "/usr/share/ats_comments/ats_comments/db/comments.db",
//         );
//     };

//     let comserv_esti_db = env::var("COMSERV_ESTIMATES_DB");
//     if comserv_esti_db.is_err() {
//         env::set_var(
//             "COMSERV_ESTIMATES_DB",
//             "/usr/share/ats_comments/ats_comments/db/estimates.db",
//         );
//     };

//     let comserv_raw_http = env::var("COMSERV_RAW_HTTP");
//     if comserv_raw_http.is_err() {
//         env::set_var("COMSERV_RAW_HTTP", "0.0.0.0");
//     };

//     let comserv_http = env::var("COMSERV_HTTP_ADDR");
//     if comserv_http.is_err() {
//         env::set_var("COMSERV_HTTP_ADDR", "https://0.0.0.0");
//     };

//     let comserv_port = env::var("COMSERV_PORT");
//     if comserv_port.is_err() {
//         env::set_var("COMSERV_PORT", "8080");
//     };

//     info!("Environment variables set")
// }
