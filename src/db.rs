use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use bytes::Bytes;

#[derive(Debug)]
pub struct DbDropGuard {
    db: Db,
}

#[derive(Debug)]
pub struct Db {
    shared: Arc<Shared>,
}

#[derive(Debug)]
struct Shared {
    state: Mutex<State>,
}

#[derive(Debug)]
struct State {
    entries: HashMap<String, Entry>,
}

#[derive(Debug)]
struct Entry {
    id: u64,
    data: Bytes,
}
