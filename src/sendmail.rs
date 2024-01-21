use crate::types;
use log::info;
use mailjet_rs::common::Recipient;
use mailjet_rs::v3::Message;
use mailjet_rs::{Client, SendAPIVersion};
use std::env;

pub async fn send_com_mail(
    com: types::FullComment,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let api_key = env::var("COMSERV_API_KEY").unwrap();
    let sec_key = env::var("COMSERV_SEC_KEY").unwrap();
    let client = Client::new(SendAPIVersion::V3, api_key.as_str(), sec_key.as_str());

    // Create your a `Message` instance with the minimum required values
    let mut message = Message::new(
        "porthose.cjsmo.cjsmo@gmail.com",
        "ATSBOT",
        Some("ATSBOT: New Comment".to_string()),
        Some("ATSBOT: New Comment".to_string()),
    );

    let html1 = format!(
        "
        <div>
            <p style='font-size:12px;font-weight:bold;'>{}</p>
            <p style='font-size:12px;font-weight:bold;'>{}</p>
            <p style='font-size:12px;font-weight:bold;'>{}</p>
            <p style='font-size:12px;font-weight:bold;'>{}</p>
        </div>
        <a href='https://atstest.xyz/accept/{}'>
            <button style='border-radius:8px;border:none;font-size:12px;font-weight:bold;color:white;background-color:green;padding:8px;margin:8px'>Accept</button>
        </a>
        <a href='https://atstest.xyz/reject/{}'>
            <button style='border-radius:8px;border:none;font-size:12px;font-weight:bold;color:white;background-color:red;padding:8px;margin:8px'>Reject</button>
        </a>
        ",
        com.name, com.email, com.rating, com.comment, com.comid, com.comid,
    );
    message.html_part = Some(html1.to_string());

    info!("Com Mail Message: {:?}", message);

    message.push_recipient(Recipient::new("porthose.cjsmo.cjsmo@gmail.com"));

    // Finally send the message using the `Client`
    let _response = client.send(message).await;

    Ok(())
}


pub async fn send_esti_mail(
    esti: types::Estimate,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("Esti VEC: {:?}", esti);
    // Create an instance of the Mailjet API client
    // used to send the `Message` and also define your API
    // credentials
    let api_key = env::var("COMSERV_API_KEY").unwrap();
    let sec_key = env::var("COMSERV_SEC_KEY").unwrap();
    let client = Client::new(SendAPIVersion::V3, api_key.as_str(), &sec_key.as_str());

    // Create your a `Message` instance with the minimum required values
    // let sender = "porthose.cjsmo.cjsmo@gmail.com";
    let mut message = Message::new(
        "porthose.cjsmo.cjsmo@gmail.com",
        "ATSBOT",
        Some("ATSBOT: New Estimate Request".to_string()),
        Some("ATSBOT: New Estimate Request".to_string()),
    );

    let html1 = format!(
        "
        <div>
            <p style='font-size:12px;font-weight:bold;'>{}</p>
            <p style='font-size:12px;font-weight:bold;'>
                <a href='tel:{}'>{}</a>
            </p>
            <p style='font-size:12px;font-weight:bold;'>
                <a href='mailto:{}'>{}</a>
            </p>
            <p style='font-size:12px;font-weight:bold;'>{}</p>
            <p style='font-size:12px;font-weight:bold;'>{}</p>
            <p style='font-size:12px;font-weight:bold;'>{}</p>
            <p style='font-size:12px;font-weight:bold;'>{}</p>
        </div>
        <a href='https://atstest.xyz/completed/{}'>
            <button style='border-radius:8px;border:none;font-size:12px;font-weight:bold;color:white;background-color:green;padding:8px;margin:8px'>Completed</button>
        </a>
        ",
        esti.name, esti.phone, esti.phone, esti.email, esti.email, esti.address, esti.city, esti.comment, esti.reqdate, esti.estid,
    );

    message.html_part = Some(html1.to_string());

    info!("Esti Mail Message: {:?}", message);

    // message.push_recipient(Recipient::new("alpha.treeservicecdm@gmail.com"));
    message.push_recipient(Recipient::new("porthose.cjsmo.cjsmo@gmail.com"));

    // Finally send the message using the `Client`
    let _response = client.send(message).await;

    Ok(())
}
