use googletest::matcher::MatcherResult;
use googletest::prelude::*;
use protobuf::{AbsentField, Optional, PresentField, ProxiedWithPresence};
use std::marker::PhantomData;

/// ===============================
///               IS_UNSET
/// ===============================
pub fn is_unset<'a, T: std::fmt::Debug + ProxiedWithPresence + ?Sized + 'a>(
) -> impl Matcher<ActualT = Optional<PresentField<'a, T>, AbsentField<'a, T>>> {
    UnsetMatcher::<T> {
        _phantom: PhantomData,
    }
}

struct UnsetMatcher<'a, T: ProxiedWithPresence + ?Sized> {
    _phantom: PhantomData<PresentField<'a, T>>,
}

impl<'a, T: std::fmt::Debug + ProxiedWithPresence + ?Sized> Matcher for UnsetMatcher<'a, T> {
    type ActualT = Optional<PresentField<'a, T>, AbsentField<'a, T>>;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
        actual.is_unset().into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Match => "is not set".to_string(),
            MatcherResult::NoMatch => "is set".to_string(),
        }
    }
}

/// ===============================
///               IS_SET
/// ===============================
pub fn is_set<'a, T: std::fmt::Debug + ProxiedWithPresence + ?Sized + 'a>(
) -> impl Matcher<ActualT = Optional<PresentField<'a, T>, AbsentField<'a, T>>> {
    SetMatcher::<T> {
        _phantom: PhantomData,
    }
}

struct SetMatcher<'a, T: ProxiedWithPresence + ?Sized> {
    _phantom: PhantomData<PresentField<'a, T>>,
}

impl<'a, T: std::fmt::Debug + ProxiedWithPresence + ?Sized> Matcher for SetMatcher<'a, T> {
    type ActualT = Optional<PresentField<'a, T>, AbsentField<'a, T>>;

    fn matches(&self, actual: &Self::ActualT) -> MatcherResult {
        actual.is_set().into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> String {
        match matcher_result {
            MatcherResult::Match => "is set".to_string(),
            MatcherResult::NoMatch => "is not set".to_string(),
        }
    }
}

use protobuf::View;
use std::fmt::Debug;

struct IsSetWithMatcher<'a, Proto: ?Sized + ProxiedWithPresence, Inner>(
    Inner,
    PhantomData<protobuf::FieldEntry<'a, Proto>>,
);

impl<
        'a,
        Proto: std::fmt::Debug + ProxiedWithPresence + ?Sized + 'a,
        Inner: Matcher<ActualT = View<'a, Proto>>,
    > Matcher2<'a> for IsSetWithMatcher<'a, Proto, Inner>
{
    type ActualT = protobuf::FieldEntry<'a, Proto>;

    fn matches(&self, actual: &'a Self::ActualT) -> MatcherResult {
        if actual.is_set() {
            self.0.matches(&actual.get())
        } else {
            false.into()
        }
    }
}

trait Matcher2<'a> {
    type ActualT : Debug + ?Sized + 'a;

    fn matches(&self, actual: &'a Self::ActualT) -> MatcherResult;
}