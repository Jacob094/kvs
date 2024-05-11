#![allow(dead_code)]
use std::collections::BTreeMap;

pub struct Topic<T, U> {
    topic: String,
    btree: BTreeMap<T, U>,
}

impl<T, U> Topic<T, U> {
    fn topic_init(topic: String) -> Topic<T, U> {
        return Topic {
            topic,
            btree: BTreeMap::new(),
        };
    }
}
