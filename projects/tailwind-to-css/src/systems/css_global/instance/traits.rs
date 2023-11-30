use super::*;

impl Eq for CssInstance {}

impl PartialEq<Self> for CssInstance {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id) && self.media.eq(&other.media)
    }
}

impl PartialOrd<Self> for CssInstance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.media.partial_cmp(&other.media) {
            None | Some(Ordering::Equal) => {
                self.id.partial_cmp(&other.id)
            }
            ordering => ordering
        }
    }
}

impl Ord for CssInstance {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.media.cmp(&other.media) {
            Ordering::Equal => self.id.cmp(&other.id),
            ordering => ordering
        }
    }
}
