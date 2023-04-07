use leptos::leptos_dom::helpers::TimeoutHandle;
use leptos::*;
use std::time::Duration;

/// Debounce a function call
///
/// Usage:
///
/// ```rust,ignore
/// let mut timeout: Option<::leptos::leptos_dom::helpers::TimeoutHandle> = None;
///
/// debounce(
///    &mut timeout,
///    500,
///    Box::new(move || {
///       // Do something
///    }),
/// );
/// ```
///
pub fn debounce(
    timeout: &mut Option<TimeoutHandle>,
    wait: u64,
    f: Box<dyn FnMut()>,
) {
    if timeout.is_some() {
        timeout.unwrap().clear();
    }
    *timeout =
        Some(set_timeout_with_handle(f, Duration::from_millis(wait)).unwrap());
}
