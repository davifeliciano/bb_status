use crate::utils;
use lettre::{
    message::{Attachment, Body},
    transport::smtp::authentication::Credentials,
    Message, SmtpTransport, Transport,
};

struct SmtpConfig {
    username: String,
    password: String,
    server: String,
}

impl SmtpConfig {
    fn build_from_env() -> SmtpConfig {
        let vars = ["BB_SMTP_USER", "BB_SMTP_PWD", "BB_SMTP_SERVER"];

        let [username, password, server] = vars.map(|var| {
            std::env::var(var).unwrap_or_else(|err| {
                utils::print_err_and_exit(
                    Box::new(err),
                    Some(&format!("while reading {var} env var")),
                )
            })
        });

        SmtpConfig {
            username,
            password,
            server,
        }
    }
}

fn get_attachment_filename() -> String {
    format!(
        "bb_status_{}.png",
        chrono::Local::now().format("%d_%m_%Y").to_string()
    )
}

fn build_message(to: &str, image_bytes: Vec<u8>) -> Result<Message, Box<dyn std::error::Error>> {
    let image_body = Body::new(image_bytes);
    let attachment =
        Attachment::new(get_attachment_filename()).body(image_body, "image/png".parse().unwrap());

    Ok(Message::builder()
        .from("BB Status <nobody@status.bb>".parse()?)
        .reply_to("BB Status <nobody@status.bb>".parse()?)
        .to(to.parse()?)
        .subject("Today BB status report")
        .singlepart(attachment)?)
}

pub fn mail_image(to: &str, image_bytes: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
    let message = build_message(to, image_bytes)?;
    let smtp_config = SmtpConfig::build_from_env();
    let creds = Credentials::new(smtp_config.username, smtp_config.password);
    let mailer = SmtpTransport::relay(&smtp_config.server)?
        .credentials(creds)
        .build();

    Ok(mailer.send(&message).map(|_| ())?)
}
