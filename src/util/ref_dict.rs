use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::vec::Vec;

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

    pub fn add_value(&mut self, key: &K, value: V) -> Result<(), RefDictError> {
        if self.contains(&key) {
            return Err(RefDictError::ValueAlreadyExists);
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

    pub fn add_reference(&mut self, key: &K) -> Result<usize, RefDictError> {
        if !self.contains(key) {
            return Err(RefDictError::ValueDoesNotExist)
        }
        self.indexes.get_mut(key).unwrap().references += 1;
        Ok(self.indexes[key].id)
    }

    pub fn remove_reference(&mut self, key: &K) -> Result<(), RefDictError> {
        if !self.contains(key) {
            return Err(RefDictError::ValueDoesNotExist)
        }
        self.indexes.get_mut(key).unwrap().references -= 1;
        if self.indexes[key].references == 0 {
            self.indexes.remove(key);
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum RefDictError {
    ValueAlreadyExists,
    ValueDoesNotExist,
}

struct RefDictValue {
    id: usize,
    references: usize,
}
