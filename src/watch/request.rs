use std::collections::HashMap;

#[derive(Clone)]
pub struct WatcherRequest {
    pub(crate) table_name: &'static [u8],
    pub(crate) primary_key_value: Vec<u8>,
    pub(crate) secondary_keys_value: HashMap<&'static str, Vec<u8>>,
}

impl WatcherRequest {
    pub fn new(
        table_name: &'static str,
        primary_key_value: Vec<u8>,
        secondary_keys_value: HashMap<&'static str, Vec<u8>>,
    ) -> Self {
        Self {
            table_name: table_name.as_bytes(),
            primary_key_value,
            secondary_keys_value,
        }
    }
}
