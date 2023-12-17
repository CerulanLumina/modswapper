use std::ops::Deref;

use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use once_cell::sync::Lazy;

static MATCHER: Lazy<SkimMatcherV2> = Lazy::new(|| SkimMatcherV2::default().smart_case());

pub fn fuzzy_matcher() -> &'static impl FuzzyMatcher {
    MATCHER.deref()
}
