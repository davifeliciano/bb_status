pub use clap::{Args, Parser};
use regex::Regex;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(flatten)]
    pub targets: Targets,

    #[arg(value_parser = cpf_parser, help = "CPF to search for")]
    pub cpf: String,

    #[arg(
        short,
        long,
        value_parser = scale_factor_parser,
        default_value_t = 1.2,
        help = "Final image scale factor: 1.0 leads to a 800x620 image"
    )]
    pub scale_factor: f64,
}

#[derive(Args)]
#[group(required = true, multiple = true)]
pub struct Targets {
    #[arg(short, long, value_parser = clap::value_parser!(PathBuf))]
    #[arg(help = "Path to save the image file")]
    pub output: Option<PathBuf>,

    #[arg(short, long, help = "Email to send the image file as attachment")]
    pub email: Option<String>,
}

fn cpf_parser(cpf: &str) -> Result<String, &'static str> {
    let cpf_regex = Regex::new(r"^\d{11}$").unwrap();

    if !cpf_regex.is_match(cpf) {
        return Err("a valid CPF consists of 11 digits");
    }

    Ok(cpf.to_string())
}

fn scale_factor_parser(s: &str) -> Result<f64, &'static str> {
    let scale_factor = s.parse().map_err(|_| "expected a number")?;

    if scale_factor < 1.0 {
        return Err("scale factor must be >= 1.0");
    }

    Ok(scale_factor)
}
