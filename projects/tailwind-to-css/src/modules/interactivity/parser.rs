use super::*;

impl TailwindAccentColor {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindAppearance {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindCursor {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindCaretColor {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindPointerEvents {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindResize {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindScroll {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindSnap {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl TailwindTorch {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}

impl SelectKind {
    /// https://developer.mozilla.org/zh-CN/docs/Web/CSS/user-select
    pub fn parse(pattern: &[&str]) -> Result<Self> {
        let out = match pattern {
            ["none"] => Self::None,
            ["auto"] => Self::Auto,
            ["text"] => Self::Text,
            ["contain"] => Self::Contain,
            ["all"] => Self::All,
            ["inherit"] => Self::Global(CssBehavior::Inherit),
            ["initial"] => Self::Global(CssBehavior::Initial),
            ["unset"] => Self::Global(CssBehavior::Unset),
            _ => return syntax_error!("Unknown hue-rotate instructions"),
        };
        Ok(out)
    }
}

impl TailwindSelect {
    /// https://tailwindcss.com/docs/user-select
    #[inline]
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        debug_assert!(arbitrary.is_empty(), "forbidden arbitrary in select");
        Ok(Self { kind: SelectKind::parse(pattern)? })
    }
}

impl TailwindWillChange {
    pub fn parse(pattern: &[&str], arbitrary: &str) -> Result<Self> {
        todo!()
    }
    pub fn parse_arbitrary(arbitrary: &str) -> Result<Self> {
        todo!()
    }
}