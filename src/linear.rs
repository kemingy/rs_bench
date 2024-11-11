use std::collections::HashMap;

use heapless::LinearMap;
use micromap::Map;
use small_map::SmallMap;

pub trait LinearMapTrait {
    fn new() -> Self;
    fn insert(&mut self, key: u32, value: String);
    fn get(&self, key: u32) -> Option<&String>;
}

pub struct DefaultMapStruct<const N: usize> {
    map: HashMap<u32, String>,
}

impl<const N: usize> LinearMapTrait for DefaultMapStruct<N> {
    fn new() -> Self {
        Self {
            map: HashMap::<u32, String>::with_capacity(N),
        }
    }

    fn insert(&mut self, key: u32, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: u32) -> Option<&String> {
        self.map.get(&key)
    }
}

pub struct MicroMapStruct<const N: usize> {
    map: Map<u32, String, N>,
}

impl<const N: usize> LinearMapTrait for MicroMapStruct<N> {
    fn new() -> Self {
        Self {
            map: Map::<u32, String, N>::new(),
        }
    }

    fn insert(&mut self, key: u32, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: u32) -> Option<&String> {
        self.map.get(&key)
    }
}

pub struct SmallMapStruct<const N: usize> {
    map: SmallMap<N, u32, String>,
}

impl<const N: usize> LinearMapTrait for SmallMapStruct<N> {
    fn new() -> Self {
        Self {
            map: SmallMap::<N, u32, String>::default(),
        }
    }

    fn insert(&mut self, key: u32, value: String) {
        self.map.insert(key, value);
    }

    fn get(&self, key: u32) -> Option<&String> {
        self.map.get(&key)
    }
}

pub struct LinearMapStruct<const N: usize> {
    map: LinearMap<u32, String, N>,
}

impl<const N: usize> LinearMapTrait for LinearMapStruct<N> {
    fn new() -> Self {
        Self {
            map: LinearMap::<u32, String, N>::new(),
        }
    }

    fn insert(&mut self, key: u32, value: String) {
        self.map.insert(key, value).unwrap();
    }

    fn get(&self, key: u32) -> Option<&String> {
        self.map.get(&key)
    }
}
