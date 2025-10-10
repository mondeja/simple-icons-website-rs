use anyhow::{Ok, Result};
use cucumber::given;
use end2end_helpers::AppWorld;

#[given(regex = "I see the (index|preview generator) page")]
async fn open_a_page(world: &mut AppWorld, page: String) -> Result<()> {
    let (path, selector) = match page.as_str() {
        "index" => ("", "header"),
        "preview generator" => ("/preview", ".preview"),
        _ => unreachable!(),
    };

    _ = world
        .goto_path(path)
        .await?
        .driver()
        .query(By::Css(selector))
        .wait(Duration::from_secs(60), Duration::from_millis(50))
        .and_displayed()
        .first()
        .await?;
    Ok(())
}
