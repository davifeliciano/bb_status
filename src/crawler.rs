use chromiumoxide::{
    cdp::browser_protocol::page::CaptureScreenshotFormat, handler::viewport::Viewport,
    page::ScreenshotParams, Browser, BrowserConfig,
};
use futures::StreamExt;
use regex::Regex;

struct DefaultViewport {
    width: u32,
    height: u32,
}

const DEFAULT_VIEWPORT: DefaultViewport = DefaultViewport {
    width: 800,
    height: 620,
};

pub async fn screenshot(
    cpf: &str,
    scale_factor: f64,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    if scale_factor < 1.0 {
        return Err("scale factor must be >= 1.0".into());
    }

    let viewport = Viewport {
        width: DEFAULT_VIEWPORT.width,
        height: DEFAULT_VIEWPORT.height,
        device_scale_factor: Some(scale_factor),
        ..Viewport::default()
    };

    let config = BrowserConfig::builder().viewport(viewport).build()?;
    let (mut browser, mut handler) = Browser::launch(config).await?;

    let handle = async_std::task::spawn(async move {
        while let Some(h) = handler.next().await {
            if h.is_err() {
                break;
            }
        }
    });

    let url = "https://www37.bb.com.br/portalbb/resultadoConcursos/resultadoconcursos/arh0.bbx";
    let page = browser.new_page(url).await?;

    page.find_element(r"input#formulario\:cpfPesquisa")
        .await?
        .click()
        .await?
        .type_str(cpf)
        .await?
        .press_key("Enter")
        .await?;

    let success_url_regex = Regex::new("arh0_detalhe")?;
    let url = page.wait_for_navigation().await?.url().await?.unwrap();

    if !success_url_regex.is_match(&url) {
        return Err("no result for given CPF".into());
    }

    let screenshot_params = ScreenshotParams::builder()
        .format(CaptureScreenshotFormat::Png)
        .build();

    let image_bytes = page.screenshot(screenshot_params).await?;

    browser.close().await?;
    handle.await;

    Ok(image_bytes)
}
