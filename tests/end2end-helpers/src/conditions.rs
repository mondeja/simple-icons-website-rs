use thirtyfour::{ElementPredicate, prelude::*, stringmatch::Needle};

/// Predicate that returns true for elements that have the specified inner HTML.
pub fn element_has_inner_html<N>(value: N) -> impl ElementPredicate
where
    N: Needle + Clone + Send + Sync + 'static,
{
    move |elem: WebElement| {
        let value = value.clone();
        async move { elem.inner_html().await.map(|x| value.is_match(&x)) }
    }
}
