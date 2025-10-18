use anyhow::{Ok, Result};
use cucumber::given;
use end2end_helpers::AppWorld;
use thirtyfour::prelude::*;

#[given(regex = "I see the (index|preview generator) page")]
async fn open_a_page(world: &mut AppWorld, page: String) -> Result<()> {
    let (path, selector) = match page.as_str() {
        "index" => ("", "header"),
        _ => ("/preview", ".preview"),
    };

    _ = world
        .goto_path(path)
        .await?
        .driver()
        .query(By::Css(selector))
        .and_displayed()
        .first()
        .await?;
    Ok(())
}
