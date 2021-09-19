use super::*;

///
// #[doc = include_str!("position.md")]
#[derive(Copy, Clone, Debug)]
pub struct TailwindPosition {
    kind: String,
}

impl<T> From<T> for TailwindPosition
where
    T: Into<String>,
{
    fn from(kind: T) -> Self {
        Self { kind: kind.into() }
    }
}

impl TailwindPosition {
    /// https://tailwindcss.com/docs/position
    pub fn parse(pattern: &[&str], arbitrary: &TailwindArbitrary) -> Result<Self> {
        debug_assert!(arbitrary.is_none(), "forbidden arbitrary after position");
        let kind = pattern.join("-");
        debug_assert!(Self::check_valid(&kind));
        Ok(Self { kind })
    }
    /// https://developer.mozilla.org/en-US/docs/Web/CSS/position#syntax
    pub fn check_valid(mode: &str) -> bool {
        let set = BTreeSet::from_iter(vec![
            "static", "relative", "absolute", "fixed", "sticky", "inherit", "initial", "revert", "unset",
        ]);
        set.contains(mode)
    }
}
