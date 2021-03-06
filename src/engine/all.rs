use std::fmt::{Display, Error, Formatter};
use std::sync::Arc;

use crate::item::{ItemWrapper, MatchedItem, MatchedRange, Rank};
use crate::MatchEngine;

//------------------------------------------------------------------------------
#[derive(Debug)]
pub struct MatchAllEngine {}

impl MatchAllEngine {
    pub fn builder() -> Self {
        Self {}
    }

    pub fn build(self) -> Self {
        self
    }
}

impl MatchEngine for MatchAllEngine {
    fn match_item(&self, item: Arc<ItemWrapper>) -> Option<MatchedItem> {
        let rank = Rank {
            score: 0,
            index: item.get_index() as i64,
            begin: 0,
            end: 0,
        };

        Some(
            MatchedItem::builder(item)
                .rank(rank)
                .matched_range(MatchedRange::ByteRange(0, 0))
                .build(),
        )
    }
}

impl Display for MatchAllEngine {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "Noop")
    }
}
