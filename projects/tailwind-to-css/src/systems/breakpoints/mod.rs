mod traits;
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};


#[derive(Clone, Debug)]
pub struct BreakPointSystem {
    inner: HashMap<String, BreakPoint>,
}


impl BreakPointSystem {
    /// Builtin ranges
    /// https://tailwindcss.com/docs/screens
    pub fn builtin() -> Self {
        let mut new = Self::default();
        new.register("sm".to_string(), 640);
        new.register("md".to_string(), 768);
        new.register("lg".to_string(), 1024);
        new.register("xl".to_string(), 1280);
        new.register("2xl".to_string(), 1536);
        return new;
    }

    #[inline]
    pub fn register(&mut self, name: String, width: usize) -> Option<BreakPoint> {
        self.inner.insert(name, BreakPoint { width })
    }
}

#[derive(Clone, Debug)]
pub struct BreakPoint {
    /// min-width
    /// unit: px
    width: usize,
}

