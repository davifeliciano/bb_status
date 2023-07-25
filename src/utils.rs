use std::path::PathBuf;
use text_colorizer::*;

pub async fn write_image(
    output: PathBuf,
    image_bytes: &[u8],
) -> Result<(), Box<dyn std::error::Error>> {
    async_std::fs::write(output, image_bytes).await?;

    Ok(())
}

pub fn print_err_and_exit(err: Box<dyn std::error::Error>, msg: Option<&str>) -> ! {
    match msg {
        Some(s) => eprintln!("{} {}: {}", "error:".red().bold(), s, err),
        None => eprintln!("{} {}", "error:", err),
    }

    std::process::exit(1)
}
