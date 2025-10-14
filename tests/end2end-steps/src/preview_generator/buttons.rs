use anyhow::{Ok, Result};
use cucumber::when;
use end2end_helpers::AppWorld;
use thirtyfour::prelude::*;

#[when(
    regex = r#"I upload the file "([^"]+)" by clicking the "Upload SVG" button"#
)]
async fn upload_file_by_clicking_on_upload_svg_button(
    world: &mut AppWorld,
    filepath: String,
) -> Result<()> {
    let input_id = "preview-upload-svg-button";
    let input = world.driver().find(By::Id(input_id)).await?;

    let rooted_path = std::env::current_dir()
        .unwrap()
        .join("../..")
        .join(filepath)
        .to_str()
        .unwrap()
        .to_string();
    let canonical_path = std::fs::canonicalize(&rooted_path).unwrap();
    let abs_path = canonical_path.to_str().unwrap();
    input.send_keys(abs_path).await?;
    Ok(())
}
