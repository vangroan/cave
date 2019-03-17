//! Pigeonhole Sort
#![allow(dead_code)]

use std::fmt::Debug;
use std::collections::BTreeMap;
use std::collections::btree_map;
use std::mem;

pub const HOLE_CAPACITY : u8 = 32;

pub struct PigeonholeSort<T: Default + Debug> {
    min: i32,
    max: i32,
    range: usize,

    /// The buckets where elements will be sorted into
    /// 
    /// The first item of the tuple is a cursor that points to the next open
    /// element in the array.
    holes: Vec<(u8, [T; HOLE_CAPACITY as usize])>,

    /// Emergency buckets used when the main container goes over capacity
    overflow : BTreeMap<usize, Vec<T>>,
}

impl<T> PigeonholeSort<T>
where
    T : Default + Debug
{
    pub fn new(min: i32, max: i32) -> Self {
        let sort_min = ::std::cmp::min(min, max);
        let sort_max = ::std::cmp::max(min, max);
        let range = ((max - min).abs() + 1) as usize;
        let mut holes = Vec::with_capacity(range);

        // Iterating so that we don't have to impose Copy or Clone on T
        for _ in 0..range {
            holes.push((0, Default::default()));
        }

        PigeonholeSort {
            min: sort_min,
            max: sort_max,
            range,
            holes,
            overflow: BTreeMap::new(),
        }
    }

    pub fn sort_into<F>(&mut self, source: &mut [T], target: &mut [T], extract_key: F)
    where
        F : Fn(&T) -> i32
    {
        assert_eq!(source.len(), target.len(), "Source and target slices must be the same length");

        let mut in_overflow = false;

        // Sort source into holes
        for i in 0..source.len() {
            let index = (extract_key(&source[i]) - self.min) as usize;
            let cursor = self.holes[index].0;

            if cursor >= HOLE_CAPACITY {
                // If we're over the capacity, we ignore need to fall back on the emergency buckets.
                let mut el : T = Default::default();
                mem::swap(&mut source[i], &mut el);
                self.overflow.entry(index)
                    .or_insert(vec![])
                    .push(el);
                in_overflow = true;
            } else {
                mem::swap(&mut source[i], &mut self.holes[index].1[cursor as usize]);
                self.holes[index].0 = cursor + 1;
            }
        }

        // Read contents from buckets into target
        let mut k : usize = 0;
        for i in 0..self.range {
            let count = self.holes[i].0 as usize;

            if count == 0 {
                // Empty bucket
                continue;
            }

            for j in 0..count {
                // println!("swapping hole {:?}", self.holes[i].1[j]);
                mem::swap(&mut self.holes[i].1[j], &mut target[k]);
                k += 1;
            }

            if in_overflow {
                if let btree_map::Entry::Occupied(mut e) = self.overflow.entry(i) {
                    for el in e.get_mut().drain(0..) {
                        target[k] = el;
                        k += 1;
                    }
                }
            }
        }

        // Clear hole container for next invocation
        for i in 0..self.holes.len() {
            self.holes[i] = Default::default();
        }

        self.overflow.clear();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type Depth = i32;
    type EntityId = i64;

    #[test]
    fn test_basic_sort() {
        let max_depth = 128 + 128 + 128;
        let mut sorter = PigeonholeSort::<(Depth, EntityId)>::new(0, max_depth);

        let mut source : Vec<(Depth, EntityId)> = vec![
            (1, 10001),
            (6, 10002),
            (9, 10003),
            (5, 10004),
            (4, 10005),
            (6, 10006),
        ];
        let mut target : Vec<(Depth, EntityId)> = vec![(0, 0); source.len()];

        sorter.sort_into(&mut source, &mut target, |pair| pair.0);

        assert_eq!((1, 10001), target[0]);
        assert_eq!((4, 10005), target[1]);
        assert_eq!((5, 10004), target[2]);
        assert_eq!((6, 10002), target[3]);
        assert_eq!((6, 10006), target[4]);
        assert_eq!((9, 10003), target[5]);
    }
}
