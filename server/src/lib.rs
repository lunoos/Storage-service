use std::collections::HashMap;

/// Simple in-memory storage mapping bucket -> (key -> bytes).
pub struct MyStorage {
    map: HashMap<String, HashMap<String, Vec<u8>>>,
}

impl MyStorage {
    /// Create an empty storage instance.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
}

