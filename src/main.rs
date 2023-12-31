use bb_status::cli::Cli;
use bb_status::crawler;
use bb_status::mailer;
use bb_status::utils;
use clap::Parser;

#[async_std::main]
async fn main() {
    let cli = Cli::parse();

    let image_bytes = crawler::screenshot(&cli.cpf, cli.scale_factor)
        .await
        .unwrap_or_else(|err| {
            utils::print_err_and_exit(err, Some("something went wrong while crawling page"))
        });

    if let Some(output) = cli.targets.output {
        utils::write_image(output, &image_bytes)
            .await
            .unwrap_or_else(|err| {
                utils::print_err_and_exit(err, Some("something went wrong while writing image"))
            });
    }

    if let Some(to) = cli.targets.email {
        mailer::mail_image(&to, image_bytes).unwrap_or_else(|err| {
            utils::print_err_and_exit(
                err,
                Some(&format!("something went wrong while mailing {to}")),
            )
        });
    }
}
