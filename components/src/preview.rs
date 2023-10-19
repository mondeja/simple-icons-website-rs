use crate::button::Button;
use crate::controls::download::download;
use crate::controls::search::fuzzy::search;
use crate::fetch::fetch_text_forcing_cache;
use crate::grid::ICONS;
use i18n::{move_tr, tr};
use leptos::{html::Input, *};
use simple_icons::{color, sdk::normalize_color, sdk::title_to_slug};
use simple_icons_macros::{get_number_of_icons, simple_icon_svg_path};
use std::collections::HashMap;
use types::SimpleIcon;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures;

/// Initial brand when the preview is loaded
fn initial_brand_value() -> String {
    "Simple Icons".to_string()
}

/// Initial color when the preview is loaded
fn initial_color() -> String {
    "111111".to_string()
}

/// Initial SVG path when the preview is loaded
fn initial_path() -> String {
    simple_icon_svg_path!("simpleicons").to_string()
}

/// Check if a string is a valid hex color
fn is_valid_hex_color(value: &str) -> bool {
    if value.len() != 6 && value.len() != 3 {
        return false;
    }

    for c in value.chars() {
        if !c.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

/// Get the URL of a badge
fn badge_url(slug: &str, color: &str, svg: &str, style: &str) -> String {
    format!(
        "https://img.shields.io/badge/{}-preview-{}.svg?style={}&logo=data:image/svg%2bxml;base64,{}",
        slug,
        color,
        style,
        window().btoa(svg).unwrap(),
    )
}

/// Get the contrast color for a given hex color
fn contrast_color_for(hex: &str) -> String {
    if !is_valid_hex_color(hex) {
        return "black".to_string();
    }
    let is_light_hex =
        color::is_relatively_light_icon_hex(&normalize_color(hex));
    if is_light_hex { "black" } else { "white" }.to_string()
}

/// Build a SVG string with the 24px24 viewBox and an optional `fill` attribute
fn build_svg(path: &str, fill: Option<&str>) -> String {
    format!(
        "<svg role=\"img\" viewBox=\"0 0 24 24\" xmlns=\"http://www.w3.org/2000/svg\"><path d=\"{}\"{}/></svg>",
        path,
        match fill {
            Some(fill) => format!(" fill=\"#{}\"", fill),
            None => "".to_string(),
        }
    )
}

fn path_from_simple_icon_svg(svg: &str) -> String {
    svg.split(" d=\"")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .to_string()
}

enum PreviewButtonSvgPath {
    Upload,
    Download,
    Save,
}

impl PreviewButtonSvgPath {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Upload => "M9,16V10H5L12,3L19,10H15V16H9M5,20V18H19V20H5",
            Self::Download => "M5,20H19V18H5M19,9H15V3H9V9H5L12,16L19,9",
            Self::Save => "M15,9H5V5H15M12,19A3,3 0 0,1 9,16A3,3 0 0,1 12,13A3,3 0 0,1 15,16A3,3 0 0,1 12,19M17,3H5C3.89,3 3,3.9 3,5V19A2,2 0 0,0 5,21H19A2,2 0 0,0 21,19V7L17,3Z",
        }
    }
}

fn get_preview_canvas_context() -> web_sys::CanvasRenderingContext2d {
    let container = document()
        .get_elements_by_class_name("preview-body")
        .item(0)
        .unwrap();
    let figure = container.dyn_into::<web_sys::HtmlElement>().unwrap();
    let canvas = figure
        .get_elements_by_tag_name("canvas")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.set_font("1rem sans");
    ctx
}

macro_rules! draw_badge_impl {
    ($badge_index:literal, $x:literal, $y:literal) => {{
        let badges_containers = document()
            .get_elements_by_class_name("preview-badges")
            .item(0)
            .unwrap()
            .children();
        let badge_img = badges_containers
            .item($badge_index)
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
            .first_element_child()
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        let badge_url = badge_img.src();

        let badge_img_for_canvas = document()
            .create_element("img")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        badge_img_for_canvas
            .set_attribute("style", "display: none")
            .unwrap();
        badge_img_for_canvas
            .set_attribute(
                "id",
                &format!("preview-badge-image-for-canvas-{}", $badge_index),
            )
            .unwrap();
        badge_img_for_canvas.set_cross_origin(Some("anonymous"));

        document()
            .body()
            .unwrap()
            .append_child(&badge_img_for_canvas)
            .unwrap();

        let closure: Closure<dyn FnMut()> = Closure::new(move || {
            let img = document()
                .get_element_by_id(&format!(
                    "preview-badge-image-for-canvas-{}",
                    $badge_index
                ))
                .unwrap()
                .dyn_into::<web_sys::HtmlImageElement>()
                .unwrap();

            let ctx = get_preview_canvas_context();
            ctx.draw_image_with_html_image_element(
                &img,
                $x as f64,
                420.0 + $y as f64,
            )
            .unwrap();
            document().body().unwrap().remove_child(&img).unwrap();
        });
        badge_img_for_canvas.set_onload(Some(closure.as_ref().unchecked_ref()));
        closure.forget();

        badge_img_for_canvas
            .set_attribute("src", badge_url.as_str())
            .unwrap();
    }};
}

/// Draw the current badges in the canvas
fn update_badges_in_canvas() {
    draw_badge_impl!(0, 15, 15);
    draw_badge_impl!(1, 173, 16);
    draw_badge_impl!(2, 335, 6);
    draw_badge_impl!(3, 562, 15);

    draw_badge_impl!(4, 15, 41);
    draw_badge_impl!(5, 173, 41);
    draw_badge_impl!(6, 335, 39);
    draw_badge_impl!(7, 560, 41);
}

/// Function triggered to update the canvas with the current SVG
fn update_canvas() {
    let container = document()
        .get_elements_by_class_name("preview-body")
        .item(0);
    if container.is_none() {
        return;
    }

    let figure = document()
        .get_elements_by_class_name("preview-body")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    let canvas = figure
        .get_elements_by_tag_name("canvas")
        .item(0)
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    // Draw the SVG of the preview card in the canvas
    let preview_card_svg =
        figure.get_elements_by_tag_name("svg").item(0).unwrap();
    let preview_card_img = document()
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();
    preview_card_img
        .set_attribute("style", "display: none")
        .unwrap();
    preview_card_img
        .set_attribute("id", "preview-card-image-for-canvas")
        .unwrap();
    preview_card_img.set_cross_origin(Some("anonymous"));
    document()
        .body()
        .unwrap()
        .append_child(&preview_card_img)
        .unwrap();

    // Set the onload attribute and draw the image
    let closure: Closure<dyn FnMut()> = Closure::new(move || {
        let preview_card_img = document()
            .get_element_by_id("preview-card-image-for-canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlImageElement>()
            .unwrap();
        ctx.draw_image_with_html_image_element(&preview_card_img, 0.0, 0.0)
            .unwrap();
        document()
            .body()
            .unwrap()
            .remove_child(&preview_card_img)
            .unwrap();

        update_badges_in_canvas();
    });
    preview_card_img.set_onload(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    let preview_card_url = format!(
        "data:image/svg+xml;utf8,{}",
        js_sys::encode_uri_component(&preview_card_svg.outer_html())
    );
    preview_card_img
        .set_attribute("src", preview_card_url.as_str())
        .unwrap();
}

/// Function triggered when the user uploads a SVG file
async fn on_upload_svg_file(
    file: web_sys::File,
    set_color: WriteSignal<String>,
    set_brand: WriteSignal<String>,
    set_path: WriteSignal<String>,
) {
    match wasm_bindgen_futures::JsFuture::from(file.text()).await {
        Ok(text) => {
            let value = text.as_string().unwrap();

            // Set color
            if value.contains("fill=\"") {
                let hex = normalize_color(
                    value
                        .split("fill=\"")
                        .nth(1)
                        .unwrap()
                        .split('"')
                        .next()
                        .unwrap(),
                );
                if is_valid_hex_color(&hex) {
                    set_color(hex.to_string());
                }
            }

            // Set brand
            if value.contains("<title>") && value.contains("</title>") {
                let brand = value
                    .split("<title>")
                    .nth(1)
                    .unwrap()
                    .split("</title>")
                    .next()
                    .unwrap();
                set_brand(brand.to_string());
            }

            // Set path
            if value.contains(" d=\"") {
                let path = value
                    .split(" d=\"")
                    .nth(1)
                    .unwrap()
                    .split('"')
                    .next()
                    .unwrap();
                set_path(path.to_string());
            }
        }
        Err(err) => ::log::error!("Error reading uploaded SVG file: {:?}", err),
    }
}

fn search_brand_suggestions(
    value: &str,
) -> (Vec<&'static SimpleIcon>, Vec<&'static SimpleIcon>) {
    let mut icons: Vec<&'static SimpleIcon> = Vec::with_capacity(6);
    let mut more_icons: Vec<&'static SimpleIcon> = Vec::new();
    let search_result = js_sys::Array::from(&search(value));
    let search_result_length = search_result.length();
    for i in 0..search_result_length {
        let result_icon_array = js_sys::Array::from(&search_result.get(i));
        let icon_order_alpha = result_icon_array.get(1).as_f64().unwrap();
        if i > 5 {
            more_icons.push(&ICONS[icon_order_alpha as usize]);
        } else {
            icons.push(&ICONS[icon_order_alpha as usize]);
        }
    }
    (icons, more_icons)
}

#[component]
fn BrandSuggestion(icon: &'static SimpleIcon) -> impl IntoView {
    view! {
        <li>
            <a>
                <img src=format!("./icons/{}.svg", icon.slug) width="24px" height="24px"/>
                <span>{icon.title}</span>
            </a>
        </li>
    }
}

fn on_click_brand_suggestion(
    icon: &'static SimpleIcon,
    set_brand: WriteSignal<String>,
    set_color: WriteSignal<String>,
    set_path: WriteSignal<String>,
) {
    set_brand(icon.title.to_string());
    set_color(icon.hex.to_string());
    spawn_local(async move {
        if let Some(svg) =
            fetch_text_forcing_cache(&format!("/icons/{}.svg", icon.slug)).await
        {
            set_path(path_from_simple_icon_svg(&svg));
        }

        update_canvas();
    });
}

/// Preview generator
#[component]
pub fn PreviewGenerator() -> impl IntoView {
    let (brand, set_brand) = create_signal(initial_brand_value());
    let (brand_suggestions, set_brand_suggestions) =
        create_signal(Vec::<&SimpleIcon>::with_capacity(6));
    let (more_brand_suggestions, set_more_brand_suggestions) =
        create_signal(Vec::<&SimpleIcon>::with_capacity(6));
    let (show_brand_suggestions, set_show_brand_suggestions) =
        create_signal(false);
    let (show_more_brand_suggestions, set_show_more_brand_suggestions) =
        create_signal(false);
    let brand_input_ref = create_node_ref::<Input>();

    let (color, set_color) = create_signal(initial_color());
    let color_input_ref = create_node_ref::<Input>();

    let (path, set_path) = create_signal(initial_path());
    let path_input_ref = create_node_ref::<Input>();

    // Hide the brand suggestions when the user clicks outside the input
    let body = document().body().unwrap();
    let closure: Closure<dyn FnMut(web_sys::MouseEvent)> =
        Closure::new(move |ev: web_sys::MouseEvent| {
            let target = ev
                .target()
                .unwrap()
                .dyn_into::<web_sys::HtmlElement>()
                .unwrap();
            if target.get_attribute("name").unwrap_or("".to_string())
                == "preview-brand"
            {
                return;
            }
            if target.get_attribute("class").unwrap_or("".to_string())
                == "more-suggestions"
            {
                return;
            }
            set_show_brand_suggestions(false);
            set_show_more_brand_suggestions(false);
        });
    body.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    view! {
        <div class="preview">
            <div>
                <div class="preview-input-group">
                    <label for="preview-brand">{move_tr!("brand")}</label>
                    <input
                        _ref=brand_input_ref
                        type="text"
                        class="mr-7"
                        style="width:524px"
                        name="preview-brand"
                        value=brand
                        prop:value=brand
                        on:input=move |_| {
                            set_brand(brand_input_ref.get().unwrap().value());
                            update_canvas();
                            let value = brand_input_ref.get().unwrap().value();
                            let (bs, more_bs) = search_brand_suggestions(&value);
                            let more_bs_length = more_bs.len();
                            set_brand_suggestions(bs);
                            set_more_brand_suggestions(more_bs);
                            set_show_brand_suggestions(true);
                            if value.len() < 4 || more_bs_length == 0 {
                                set_show_more_brand_suggestions(false);
                            }
                        }

                        on:focus=move |_| {
                            let value = brand_input_ref.get().unwrap().value();
                            let (bs, more_bs) = search_brand_suggestions(&value);
                            set_brand_suggestions(bs);
                            set_more_brand_suggestions(more_bs);
                            set_show_brand_suggestions(true);
                        }
                    />

                    <ul
                        class=move || {
                            let mut cls = "preview-brand-suggestions".to_string();
                            if show_more_brand_suggestions() {
                                cls.push_str(" overflow-y-scroll");
                            }
                            cls
                        }

                        class:hidden=move || {
                            !show_brand_suggestions() || brand_suggestions().is_empty()
                        }
                    >

                        {move || {
                            if !show_brand_suggestions() {
                                return vec![];
                            }
                            let mut suggestions_containers = vec![];
                            let bs = brand_suggestions();
                            for icon in bs {
                                suggestions_containers
                                    .push(
                                        view! {
                                            <BrandSuggestion
                                                icon=icon
                                                on:click=move |_| {
                                                    on_click_brand_suggestion(
                                                        icon,
                                                        set_brand,
                                                        set_color,
                                                        set_path,
                                                    );
                                                }
                                            />
                                        },
                                    );
                            }
                            if !show_more_brand_suggestions() {
                                if !more_brand_suggestions().is_empty() {
                                    suggestions_containers
                                        .push(
                                            view! {
                                                <>
                                                    <li
                                                        class="more-suggestions"
                                                        on:click=move |_| {
                                                            set_show_more_brand_suggestions(true);
                                                        }
                                                    >

                                                        <span>+</span>
                                                    </li>
                                                </>
                                            }
                                                .into(),
                                        );
                                }
                            } else {
                                let more_bs = more_brand_suggestions();
                                for icon in more_bs {
                                    suggestions_containers
                                        .push(
                                            view! {
                                                <BrandSuggestion
                                                    icon=icon
                                                    on:click=move |_| {
                                                        on_click_brand_suggestion(
                                                            icon,
                                                            set_brand,
                                                            set_color,
                                                            set_path,
                                                        );
                                                    }
                                                />
                                            },
                                        );
                                }
                            }
                            suggestions_containers
                        }}

                    </ul>
                </div>
                <div class="preview-input-group">
                    <label for="preview-color">{move_tr!("color")}</label>
                    <input
                        _ref=color_input_ref
                        type="text"
                        style="width:68px"
                        name="preview-color"
                        value=color
                        prop:value=color
                        on:input=move |_| {
                            let input = color_input_ref.get().unwrap();
                            let selection_start = input.selection_start().unwrap();
                            let selection_end = input.selection_end().unwrap();
                            let normalized_value = input.value().to_uppercase().replace('#', "");
                            input.set_value(&normalized_value);
                            input.set_selection_start(selection_start).unwrap();
                            input.set_selection_end(selection_end).unwrap();
                            set_color(normalized_value);
                            update_canvas();
                        }

                        class:invalid=move || !is_valid_hex_color(&color())
                        maxlength=6
                    />

                </div>
            </div>
            <div class="preview-input-group">
                <label for="preview-path">{move_tr!("path")}</label>
                <input
                    _ref=path_input_ref
                    type="text"
                    style="width:682px"
                    name="preview-path"
                    value=path
                    prop:value=path
                    on:input=move |_| {
                        set_path(path_input_ref.get().unwrap().value());
                        update_canvas();
                    }
                />

            </div>

            <figure class="preview-body">
                <svg
                    width="740"
                    height="420"
                    viewBox="0 0 740 420"
                    xmlns="http://www.w3.org/2000/svg"
                    class="pt-3"
                >
                    <rect
                        fill=move || format!("#{}", color())
                        height="420"
                        rx="10"
                        ry="10"
                        width="740"
                        x="0"
                        y="0"
                    ></rect>
                    <svg viewBox="0 0 24 24" width="24" height="24" x="18" y="20">
                        <path d=move || path() fill=move || contrast_color_for(&color())></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="80" height="80" x="70" y="20">
                        <path d=move || path() fill=move || contrast_color_for(&color())></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="138" height="138" x="174" y="20">
                        <path d=move || path() fill=move || contrast_color_for(&color())></path>
                    </svg>
                    <svg viewBox="0 0 24 24" width="375" height="375" x="350" y="20">
                        <path d=move || path() fill=move || contrast_color_for(&color())></path>
                    </svg>

                    <g transform="translate(21,235)" style="font-family: Helvetica">
                        <text fill=move || contrast_color_for(&color()) font-size="25">
                            {move || format!("{} Preview", brand())}
                        </text>
                        <text fill=move || contrast_color_for(&color()) font-size="17" y="25">
                            {move || format!("{}.svg", title_to_slug(&brand()))}
                        </text>
                        <text fill=move || contrast_color_for(&color()) font-size="16" y="61">
                            {move || format!("Brand: {}", brand())}
                        </text>
                        <text fill=move || contrast_color_for(&color()) font-size="16" y="84">
                            {move || format!("Color: #{}", color())}
                        </text>

                        <g transform="translate(3, 142)" style="font-family: Helvetica">
                            <svg viewBox="0 0 24 24" width="24" height="24">
                                <path
                                    d=simple_icon_svg_path!("simpleicons")
                                    fill=move || contrast_color_for(&color())
                                ></path>
                            </svg>
                            <text
                                fill=move || contrast_color_for(&color())
                                x="30"
                                y="7"
                                font-size="12"
                            >
                                {format!("{} Free SVG brand icons", get_number_of_icons!())}
                            </text>
                            <text
                                fill=move || contrast_color_for(&color())
                                x="30"
                                y="25"
                                font-size="12"
                            >
                                available at simpleicons.org
                            </text>
                        </g>
                    </g>
                </svg>
                <canvas height="490" width="740"></canvas>
            </figure>
            <div class="preview-badges">
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some("FFF")),
                        "flat",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some("FFF")),
                        "plastic",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some("FFF")),
                        "for-the-badge",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some("FFF")),
                        "flat-square",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some(&color())),
                        "flat",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some(&color())),
                        "plastic",
                    )/>
                </div>
                <div>
                    <img src=move || badge_url(
                        &title_to_slug(&brand()),
                        &color(),
                        &build_svg(&path(), Some(&color())),
                        "for-the-badge",
                    )/>
                </div>
                <div>
                    <img
                        src=move || badge_url(
                            &title_to_slug(&brand()),
                            &color(),
                            &build_svg(&path(), Some("000")),
                            "social",
                        )

                        on:load=move |_| update_canvas()
                    />
                </div>
            </div>
            <div class="preview-buttons">
                <form class="inline-block">
                    <input
                        type="file"
                        name="upload-svg"
                        accept=".svg"
                        class="absolute w-0 h-0 -z-index-1"
                        on:change=move |ev| {
                            let input = ev
                                .target()
                                .unwrap()
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .unwrap();
                            let file = input.files().unwrap().get(0).unwrap();
                            spawn_local(on_upload_svg_file(file, set_color, set_brand, set_path));
                        }
                    />

                    <Button
                        svg_path=PreviewButtonSvgPath::Upload.as_str()
                        title=move_tr!("upload-svg")
                        on:click=move |el| {
                            let input = document()
                                .query_selector("input[name='upload-svg']")
                                .unwrap()
                                .unwrap()
                                .dyn_into::<web_sys::HtmlInputElement>()
                                .unwrap();
                            input.click();
                            el.target()
                                .unwrap()
                                .dyn_into::<web_sys::HtmlElement>()
                                .unwrap()
                                .blur()
                                .unwrap();
                        }
                    />

                </form>
                <Button
                    svg_path=PreviewButtonSvgPath::Save.as_str()
                    title=move_tr!("save-preview")
                    class="float-right ml-4"
                    on:click=move |el| {
                        let figure = document()
                            .get_elements_by_class_name("preview-body")
                            .item(0)
                            .unwrap()
                            .dyn_into::<web_sys::HtmlElement>()
                            .unwrap();
                        let canvas = figure
                            .get_elements_by_tag_name("canvas")
                            .item(0)
                            .unwrap()
                            .dyn_into::<web_sys::HtmlCanvasElement>()
                            .unwrap();
                        let filename = format!("{}.png", title_to_slug(&brand()));
                        let url = canvas.to_data_url().unwrap();
                        download(&filename, &url);
                        el.target()
                            .unwrap()
                            .dyn_into::<web_sys::HtmlElement>()
                            .unwrap()
                            .blur()
                            .unwrap();
                    }
                />

                <Button
                    svg_path=PreviewButtonSvgPath::Download.as_str()
                    title=move_tr!(
                        "download-filetype", & { let mut map = HashMap::new(); map.insert("filetype"
                        .to_string(), tr!("svg") .into()); map }
                    )

                    class="float-right"
                    on:click=move |el| {
                        let filename = format!("{}.svg", title_to_slug(&brand()));
                        let url = format!(
                            "data:image/svg+xml;utf8,{}",
                            js_sys::encode_uri_component(&build_svg(&path(), None)),
                        );
                        download(&filename, &url);
                        el.target()
                            .unwrap()
                            .dyn_into::<web_sys::HtmlElement>()
                            .unwrap()
                            .blur()
                            .unwrap();
                    }
                />

            </div>
        </div>
    }
}