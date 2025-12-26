use anyhow::{Ok, Result};
use cucumber::given;
use end2end_helpers::AppWorld;
use thirtyfour::prelude::*;

#[given(
    regex = r"I see the (index|preview generator) page(?:\s+with the url params\s+(.+))?"
)]
async fn open_a_page(
    world: &mut AppWorld,
    page: String,
    url_params: String, // cucumber pasará un string vacío si no hay match
) -> Result<()> {
    let (path, selector) = match page.as_str() {
        "index" => ("", "header"),
        _ => ("/preview", ".preview"),
    };

    let path = if !url_params.is_empty() {
        format!("{path}?{url_params}")
    } else {
        path.to_string()
    };

    world
        .goto_path(&path)
        .await?
        .driver()
        .query(By::Css(selector))
        .and_displayed()
        .first()
        .await?;

    Ok(())
}
