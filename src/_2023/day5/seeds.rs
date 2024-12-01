use std::ops::{Deref, DerefMut, Range};

use crate::shared::list_util::SpaceSeparatedList;

pub struct Seeds(Vec<Range<usize>>);

impl Seeds {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl DerefMut for Seeds {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Seeds {
    type Target = Vec<Range<usize>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<&str> for Seeds {
    fn from(value: &str) -> Self {
        match value.strip_prefix("seeds: ") {
            Some(seeds_as_str) => {
                let mut seeds = Seeds::new();
                let seeds_vec = SpaceSeparatedList::from(seeds_as_str.to_owned());
                for index in 0..seeds_vec.list.len() / 2 {
                    let first = seeds_vec.list.get(index * 2).unwrap().to_owned();
                    seeds.push(Range {
                        start: first,
                        end: first + seeds_vec.list.get(index * 2 + 1).unwrap().to_owned() - 1,
                    })
                }
                seeds
            }
            _ => todo!(),
        }
    }
}

impl From<Vec<Range<usize>>> for Seeds {
    fn from(ranges: Vec<Range<usize>>) -> Self {
        let mut seeds = Seeds::new();
        for range in ranges {
            seeds.push(range);
        }
        seeds
    }
}

impl IntoIterator for Seeds {
    type Item = Range<usize>;

    type IntoIter = std::vec::IntoIter<Range<usize>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}
