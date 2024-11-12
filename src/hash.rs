use std::collections::HashMap;

use ahash::AHashMap;
use nohash_hasher::IntMap;

#[inline]
pub fn nohash(num: usize, epoch: usize) {
    let mut map = IntMap::<u32, String>::with_capacity_and_hasher(num, Default::default());

    for i in 0..num {
        map.insert(i as u32, i.to_string());
    }

    for _ in 0..epoch {
        for i in 0..num {
            map.get(&(i as u32));
        }
    }
}

#[inline]
pub fn hashmap(num: usize, epoch: usize) {
    let mut map = HashMap::<u32, String>::with_capacity(num);

    for i in 0..num {
        map.insert(i as u32, i.to_string());
    }

    for _ in 0..epoch {
        for i in 0..num {
            map.get(&(i as u32));
        }
    }
}

#[inline]
pub fn ahash(num: usize, epoch: usize) {
    let mut map = AHashMap::<u32, String>::with_capacity(num);

    for i in 0..num {
        map.insert(i as u32, i.to_string());
    }

    for _ in 0..epoch {
        for i in 0..num {
            map.get(&(i as u32));
        }
    }
}
