use crate::ui::viewmodel::{Filter, SwapSetViewModel};
use either::Either;

pub trait Filterable {
    fn filter_by(&self) -> &str;
}

impl Filterable for &mut SwapSetViewModel {
    fn filter_by(&self) -> &str {
        self.label.as_str()
    }
}

impl Filter {
    pub fn filter_iter<'a, T: Filterable + 'a>(
        &'a self,
        items: impl Iterator<Item = T> + 'a,
    ) -> impl Iterator<Item = T> + 'a {
        if self.filter.is_empty() {
            Either::Left(items)
        } else {
            Either::Right(filter_with_pattern(self.filter.as_str(), items))
        }
    }
}

#[cfg(feature = "fuzzy-matcher")]
fn filter_with_pattern<'a, T: Filterable + 'a>(
    pattern: &'a str,
    items: impl Iterator<Item = T> + 'a,
) -> impl Iterator<Item = T> + 'a {
    use crate::fuzzy::fuzzy_matcher;
    use fuzzy_matcher::FuzzyMatcher;
    let fuzzy = fuzzy_matcher();
    let mut fuzzied_vec = items
        .filter_map(|item| {
            fuzzy
                .fuzzy_match(item.filter_by(), pattern)
                .map(|score| (item, score))
        })
        .collect::<Vec<_>>();

    fuzzied_vec.sort_by(|(_, a_score), (_, b_score)| b_score.cmp(a_score));

    fuzzied_vec
        .into_iter()
        .filter(|(_, score)| *score > 0)
        .map(|(item, _)| item)
}

#[cfg(not(feature = "fuzzy-matcher"))]
fn filter_with_pattern<'a, T: Filterable>(
    pattern: &'a str,
    items: impl Iterator<Item = T> + 'a,
) -> impl Iterator<Item = T> + 'a {
    items.filter(move |a| a.filter_by().contains(pattern))
}
