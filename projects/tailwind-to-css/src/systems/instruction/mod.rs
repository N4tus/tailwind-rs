mod arbitrary;
mod display;
mod methods;
mod resolver;
pub use self::arbitrary::TailwindArbitrary;
use crate::{TailwindBuilder, *};
use css_color::Srgb;
use std::{
    fmt::{Debug, Display, Formatter},
    str::FromStr,
};
use std::fmt::Write;
use tailwind_ast::{parse_fraction, ASTVariant, AstStyle};
use crate::systems::media::Media;

/// `v:v:-a-a-[A]`
#[derive(Debug, Clone)]
pub struct TailwindInstruction {
    media: Option<Media>,
    negative: Negative,
    variants: Vec<TailwindVariant>,
    elements: TailwindElements,
    arbitrary: TailwindArbitrary,
}

#[derive(Debug, Clone, Hash)]
pub struct TailwindVariant {
    not: bool,
    pseudo: bool,
    names: Vec<String>,
}

impl TailwindVariant {
    pub fn write_css(&self, f: &mut (dyn Write)) -> Result<()> {
        if self.pseudo {
            f.write_char(':')?;
        }
        f.write_char(':')?;
        if self.not {
            f.write_str("not-")?;
        }
        let mut it = self.names.iter();
        if let Some(name) = it.next() {
            f.write_str(name)?;
        }
        for name in it {
            f.write_char('-')?;
            f.write_str(name)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct TailwindElements {
    inner: Vec<String>,
}

/// <https://github.com/tw-in-js/twind/blob/main/src/twind/variants.ts>
#[derive(Copy, Clone, Debug)]
pub enum TailwindVariantKind {
    Dark,
    Sticky,
    MotionReduce,
    MotionSafe,
    First,
    Last,
    Even,
    Odd,
    Children,
    Siblings,
    Sibling,
    Override,
}
