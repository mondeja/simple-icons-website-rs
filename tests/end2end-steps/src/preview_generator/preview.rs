use anyhow::{Ok, Result};
use cucumber::then;
use end2end_helpers::AppWorld;
use std::time::Duration;
use thirtyfour::prelude::*;

#[then(
    regex = r#"The (title|filename|brand|color) in the preview is "([^"]+)""#
)]
async fn check_preview_data(
    world: &mut AppWorld,
    subject: String,
    value: String,
) -> Result<()> {
    let selector = match subject.as_str() {
        "title" => ".preview-figure > svg > g > text:nth-child(1)",
        "filename" => ".preview-figure > svg > g > text:nth-child(2)",
        "brand" => ".preview-figure > svg > g > text:nth-child(3)",
        "color" => ".preview-figure > svg > g > text:nth-child(4)",
        _ => unreachable!(),
    };
    _ = world
        .driver()
        .query(By::Css(selector))
        .wait(Duration::from_secs(6), Duration::from_millis(50))
        .with_filter(move |element: thirtyfour::WebElement| {
            let value = value.clone();
            async move {
                let text = element.inner_html().await;
                if let std::result::Result::Ok(text) = text {
                    return std::result::Result::Ok(text == value);
                }
                std::result::Result::Ok(false)
            }
        })
        .exists()
        .await?;
    Ok(())
}

#[then(regex = "The background color of the preview is (#[0-9a-fA-F]{3,6})")]
async fn check_preview_background_color(
    world: &mut AppWorld,
    color: String,
) -> Result<()> {
    let selector = ".preview-figure > svg > rect:nth-child(1)";
    _ = world
        .driver()
        .query(By::Css(selector))
        .wait(Duration::from_secs(6), Duration::from_millis(50))
        .with_filter(move |element: thirtyfour::WebElement| {
            let color = color.clone();
            async move {
                let fill = element.attr("fill").await;
                if let std::result::Result::Ok(Some(fill)) = fill {
                    return std::result::Result::Ok(fill == color);
                }
                std::result::Result::Ok(false)
            }
        })
        .exists()
        .await?;
    Ok(())
}

#[then(regex = r#"The SVG paths of the preview (start with|are) "([^"]+)""#)]
async fn check_preview_svg_paths(
    world: &mut AppWorld,
    mode: String,
    value: String,
) -> Result<()> {
    let client = world.driver().clone();

    fn are_predicate(d: &str, value: &str) -> bool {
        d == value
    }

    fn starts_with_predicate(d: &str, value: &str) -> bool {
        d.starts_with(value)
    }

    _ = world
        .driver()
        .query(By::Css(".preview-figure > svg"))
        .wait(Duration::from_secs(6), Duration::from_millis(50))
        .with_filter(move |_| {
            let value = value.clone();
            let client = client.clone();
            let mode = mode.clone();
            async move {
                let paths_elements = client
                    .find_all(By::Css(".preview-figure > svg > svg > path"))
                    .await;
                if let std::result::Result::Ok(paths_elements) = paths_elements
                {
                    let mut paths = vec![];
                    for path_element in &paths_elements {
                        if let std::result::Result::Ok(Some(d)) =
                            path_element.attr("d").await
                        {
                            paths.push(d);
                        }
                    }
                    let predicate_fn = match mode.as_str() {
                        "are" => are_predicate,
                        _ => starts_with_predicate,
                    };
                    std::result::Result::Ok(
                        paths.iter().all(|d| predicate_fn(d, &value)),
                    )
                } else {
                    std::result::Result::Ok(false)
                }
            }
        })
        .exists()
        .await?;
    Ok(())
}
