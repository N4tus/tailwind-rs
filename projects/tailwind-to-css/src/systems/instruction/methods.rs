use super::*;

impl<'a> From<AstStyle<'a>> for TailwindInstruction {
    fn from(node: AstStyle<'a>) -> Self {
        Self {
            media: node.variants.iter().filter_map(|s| s.try_into().ok()).next(),
            negative: Negative::from(node.negative),
            variants: node.variants.into_iter().filter_map(|s| Media::try_from(s).err()).map(|s| s.into()).collect(),
            elements: TailwindElements { inner: node.elements.into_iter().map(|s| s.to_string()).collect() },
            arbitrary: TailwindArbitrary::from(node.arbitrary.unwrap_or_default()),
        }
    }
}

impl<'a> From<ASTVariant<'a>> for TailwindVariant {
    fn from(node: ASTVariant<'a>) -> Self {
        Self { not: node.not, pseudo: node.pseudo, names: node.names.into_iter().map(|s| s.to_string()).collect() }
    }
}

impl TailwindInstruction {
    #[inline]
    pub fn view_elements(&self) -> Vec<&str> {
        self.elements.inner.iter().map(|s| s.as_str()).collect()
    }
    #[inline]
    pub fn view_arbitrary(&self) -> &TailwindArbitrary {
        &self.arbitrary
    }
    // TODO
    pub fn normalization(self) -> Self {
        self
    }

    pub fn media(&self) -> Option<Media> {
        self.media
    }

    pub fn variants(&self) -> Vec<TailwindVariant> {
        self.variants.clone()
    }
}
