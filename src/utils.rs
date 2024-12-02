use std::collections::HashMap;
use std::hash::Hash;

pub fn create_array_index_by_field<T, K>(
    array: Vec<T>,
    key_selector: fn(&T) -> K,
) -> HashMap<K, Vec<T>>
where
    T: Clone,
    K: Clone + Eq + Hash,
{
    let mut index = HashMap::new();
    for item in array {
        let key = key_selector(&item);
        let entry = index.entry(key.clone()).or_insert_with(Vec::new);
        entry.push(item);
    }
    index
}

pub fn create_index_by_field<T, K>(array: Vec<T>, key_selector: fn(&T) -> K) -> HashMap<K, T>
where
    T: Clone,
    K: Clone + Eq + Hash,
{
    let mut index = HashMap::new();
    for item in array {
        let key = key_selector(&item);
        index.entry(key.clone()).or_insert(item);
    }
    index
}
