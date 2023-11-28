use tailwind_ast::ASTVariant;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum Media {
    SM,
    MD,
    LG,
    XL,
    XXL,
}

impl Media {
    pub(crate) fn px(&self) -> u32 {
        match self {
            Media::SM => 640,
            Media::MD => 768,
            Media::LG => 1024,
            Media::XL => 1280,
            Media::XXL => 1536,
        }
    }
}

impl<'s> TryFrom<&'s str> for Media {
    type Error = &'s str;

    fn try_from(value: &'s str) -> Result<Self, Self::Error> {
        match value {
            "sm" => Ok(Media::SM),
            "md" => Ok(Media::MD),
            "lg" => Ok(Media::LG),
            "xl" => Ok(Media::XL),
            "2xl" => Ok(Media::XXL),
            other => Err(other)
        }
    }
}

impl<'a> TryFrom<&'a ASTVariant<'a>> for Media {
    type Error = &'a ASTVariant<'a>;

    fn try_from(value: &'a ASTVariant<'a>) -> Result<Self, Self::Error> {
        (!value.not && !value.pseudo && value.names.len() == 1)
            .then(|| value.names[0].try_into().map_err(|_| value))
            .unwrap_or(Err(value))
    }
}

impl<'a> TryFrom<ASTVariant<'a>> for Media {
    type Error = ASTVariant<'a>;

    fn try_from(value: ASTVariant<'a>) -> Result<Self, Self::Error> {
        if !value.not && !value.pseudo && value.names.len() == 1 {
            value.names[0].try_into().map_err(|_| value)
        } else {
            Err(value)
        }
    }
}