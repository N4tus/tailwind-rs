use super::*;

impl Display for TailwindAspect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Auto => {
                write!(f, "aspect-auto")
            }
            Self::Arbitrary(a, b) => {
                write!(f, "aspect-{}/{}", a, b)
            }
        }
    }
}

impl TailwindInstance for TailwindAspect {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let ar = match self {
            Self::Auto => "auto",
            Self::Arbitrary(_, _) => {
                todo!()
            }
        };
        css_attributes! {
            "aspect-ratio" => ar
        }
    }
}

impl Display for TailwindContainer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindContainer {}

impl Display for LayoutBreakKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Before => f.write_str("break-before"),
            Self::After => f.write_str("break-after"),
            Self::Inside => f.write_str("break-inside"),
        }
    }
}

impl Display for TailwindLayoutBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.kind, self.info)
    }
}

impl TailwindInstance for TailwindLayoutBreak {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        let class = self.kind.to_string();
        let breaking = self.info.to_string();
        css_attributes! {
            class => breaking
        }
    }
}

impl Display for TailwindColumns {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindColumns {
    fn id(&self) -> String {
        todo!()
    }
}

impl Display for TailwindBoxDecorationBreak {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBoxDecorationBreak {
    fn id(&self) -> String {
        todo!()
    }
}

impl Display for TailwindBoxSizing {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindBoxSizing {}

impl Display for TailwindDisplay {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindDisplay {}

impl Display for TailwindClear {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindClear {}

impl Display for TailwindIsolation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindIsolation {}

impl Display for TailwindObjectFit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindObjectFit {}

impl Display for TailwindObjectPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindObjectPosition {}

impl Display for TailwindOverflow {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindOverflow {}

impl Display for TailwindOverscroll {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindOverscroll {}

impl Display for TailwindFloat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindFloat {}

impl Display for TailwindPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindPosition {}

impl Display for TailwindVisibility {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl TailwindInstance for TailwindVisibility {}

impl Display for TailWindZIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Positive(n) => write!(f, "z-{}", n),
            Self::Negative(n) => write!(f, "-z-{}", n),
            Self::Auto => write!(f, "z-auto"),
        }
    }
}

impl TailwindInstance for TailWindZIndex {
    fn attributes(&self, _: &TailwindBuilder) -> BTreeSet<CssAttribute> {
        match self {
            Self::Positive(n) => css_attributes! {
                "z-index" => &n.to_string()
            },
            Self::Negative(n) => css_attributes! {
                "z-index" => &(-(*n as isize)).to_string()
            },
            Self::Auto => css_attributes! {
                "z-index" => "auto"
            },
        }
    }
}