use thirtyfour::{ElementPredicate, prelude::*, stringmatch::Needle};

/// Predicate that returns true for elements that have the specified text.
/// See the `Needle` documentation for more details on text matching rules.
pub fn element_has_inner_html<N>(text: N) -> impl ElementPredicate
where
    N: Needle + Clone + Send + Sync + 'static,
{
    move |elem: WebElement| {
        let text = text.clone();
        async move { elem.inner_html().await.map(|x| text.is_match(&x)) }
    }
}
