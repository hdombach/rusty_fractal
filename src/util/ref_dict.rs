use std::collections::HashMap;
use std::hash::Hash;
use std::slice;
use std::vec::Vec;

use super::error::Error;

pub struct RefDict<K, V> {
    indexes: HashMap<K, RefDictValue>,
    elements: Vec<V>,
    free_elements: Vec<usize>,
}

impl<K, V> RefDict<K, V> {
    pub fn new() -> Self {
        Self {
            indexes: HashMap::new(),
            elements: Vec::new(),
            free_elements: Vec::new(),
        }
    }
}

impl<K: Hash + Eq + Clone, V> RefDict<K, V> {
    pub fn get_value(&self, index: usize) -> &V {
        &self.elements[index]
    }
    pub fn get_mut_value(&mut self, index: usize) -> &mut V {
        &mut self.elements[index]
    }

    pub fn get_value_id(&self, key: K) -> Option<usize> {
        if self.indexes.contains_key(&key) {
            return Some(self.indexes[&key].id);
        } else {
            return None;
        }
    }

    pub fn contains(&self, key: &K) -> bool {
        self.indexes.contains_key(key)
    }

    pub fn get_keys(&self) -> Vec<&K> {
        self.indexes.keys().collect()
    }

    pub fn add_value(&mut self, key: &K, value: V) -> Result<(), Error> {
        if self.contains(&key) {
            return Err(Error::value_already_exists());
        }
        let id;
        if self.free_elements.len() > 0 {
            id = self.free_elements.pop().unwrap();
            self.elements[id] = value;
        } else {
            id = self.elements.len();
            self.elements.push(value);
        }
        self.indexes.insert(key.clone(), RefDictValue {
            id,
            references: 0,
        });
        Ok(())
    }

    pub fn add_reference(&mut self, key: &K) -> Result<usize, Error> {
        if !self.contains(key) {
            return Err(Error::value_does_not_exist())
        }
        self.indexes.get_mut(key).unwrap().references += 1;
        Ok(self.indexes[key].id)
    }

    pub fn remove_reference(&mut self, key: &K) -> Result<(), Error> {
        if !self.contains(key) {
            return Err(Error::value_does_not_exist())
        }
        self.indexes.get_mut(key).unwrap().references -= 1;
        if self.indexes[key].references == 0 {
            self.indexes.remove(key);
        }
        Ok(())
    }
}

struct RefDictValue {
    id: usize,
    references: usize,
}
