use super::*;
use crate::{Base62, TailwindVariant};
use crate::systems::media::Media;

mod traits;

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Clone, Hash)]
pub(crate) struct CssInstance {
    pub inlineable: bool,
    pub obfuscate: bool,
    pub id: String,
    pub selector: String,
    pub attribute: CssAttributes,
    pub addition: String,
    pub media: Option<Media>,
    pub variants: Vec<TailwindVariant>,
}

// noinspection DuplicatedCode
impl CssInstance {
    pub fn new(item: &dyn TailwindInstance, ctx: &TailwindBuilder, media: Option<Media>, variants: Vec<TailwindVariant>, obfuscate: bool) -> Self {
        Self {
            obfuscate,
            inlineable: item.inlineable(),
            id: item.id(),
            selector: item.selectors(ctx),
            attribute: item.attributes(ctx),
            addition: item.additional(ctx),
            media,
            variants,
        }
    }

    pub fn obfuscate(css: &Self) -> String {
        let mut hasher = Xxh3::new();
        css.attribute.hash(&mut hasher);
        css.addition.hash(&mut hasher);
        css.media.hash(&mut hasher);
        css.selector.hash(&mut hasher);
        css.variants.hash(&mut hasher);
        hasher.finish().base62()
    }
    pub fn get_class(&self) -> String {
        match self.obfuscate {
            true => Self::obfuscate(self),
            false => self.id.clone(),
        }
    }
    /// write css to buffers
    pub fn write_css(&self, f: &mut (dyn Write)) -> Result<()> {
        if let Some(media) = self.media {
            write!(f, "@media(min-width:{}px){{", media.px())?;
        }
        f.write_char('.')?;
        normalize_class_name(f, &self.get_class())?;
        for variant in &self.variants {
            variant.write_css(f)?;
        }
        f.write_str(&self.selector)?;
        f.write_char('{')?;
        write!(f, "{}", self.attribute)?;
        f.write_char('}')?;
        write!(f, "{}", self.addition)?;
        if self.media.is_some() {
            f.write_char('}')?;
        }
        Ok(())
    }
}
