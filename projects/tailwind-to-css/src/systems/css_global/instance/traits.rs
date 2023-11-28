use super::*;

impl Eq for CssInstance {}

impl PartialEq<Self> for CssInstance {
    fn eq(&self, other: &Self) -> bool {
        self.selector.eq(&other.selector) && self.media.eq(&other.media)
    }
}

impl PartialOrd<Self> for CssInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.media.partial_cmp(&other.media) {
            None | Some(Ordering::Equal) => {
                self.selector.partial_cmp(&other.selector)
            }
            ordering => ordering
        }
    }
}

impl Ord for CssInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.media.cmp(&other.media) {
            Ordering::Equal => self.selector.cmp(&other.selector),
            ordering => ordering
        }
    }
}
