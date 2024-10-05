use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {

use spin_sdk::key_value::Store;
use anyhow::Result;

pub struct CounterService {
    store: Store,
}

impl CounterService {
    pub fn new() -> Result<Self> {
        let store = Store::open_default()?;
        Ok(Self { store })
    }

    pub async fn increment_counter(&self, key: &str) -> Result<()> {
        let count: u64 = self.store
            .get_json(key)?
            .unwrap_or_default();

        let updated_count = count + 1;

        self.store.set_json(key, &updated_count)?;
        Ok(())
    }

    pub async fn get_count(&self, key: &str) -> Result<u64> {
        let stored_count: u64 = self.store
            .get_json(key)?
            .unwrap_or_default();

        Ok(stored_count)
    }
}
}}
