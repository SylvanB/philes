use async_trait::async_trait;
use futures::lock::Mutex;
use std::hash::Hash;
use std::{collections::HashMap, sync::Arc};

#[async_trait]
pub trait KeyStore<K, V>
where
    K: Clone + Eq + Hash + Send + Sync,
    V: Clone + Default + Send + Sync,
{
    fn new() -> Self;

    async fn get(&self, key: K) -> Option<V>;

    async fn get_all(&self) -> HashMap<K, V>;

    async fn delete(&self, key: K) -> Result<(), KeyStoreError>;

    async fn upsert(&self, key: K, value: V) -> Result<(), KeyStoreError>;
}

pub(crate) struct InMemoryKeyValueStore<K, V>
where
    K: Eq + Hash + Send + Sync,
    V: Clone + Default + Send + Sync,
{
    store: Arc<Mutex<HashMap<K, V>>>,
}

#[async_trait]
impl<K, V> KeyStore<K, V> for InMemoryKeyValueStore<K, V>
where
    K: Clone + Eq + Hash + Send + Sync,
    V: Clone + Default + Send + Sync,
{
    fn new() -> InMemoryKeyValueStore<K, V> {
        InMemoryKeyValueStore::<K, V> {
            store: Arc::new(Mutex::new(HashMap::<K, V>::new())),
        }
    }

    async fn get(&self, key: K) -> Option<V> {
        match self.store.lock().await.get(&key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    async fn get_all(&self) -> HashMap<K, V> {
        self.store.lock().await.clone()
    }

    async fn delete(&self, key: K) -> Result<(), KeyStoreError> {
        match self.store.lock().await.remove(&key) {
            Some(_) => Ok(()),
            None => Err(KeyStoreError::ValueNotPresent),
        }
    }

    async fn upsert(&self, key: K, value: V) -> Result<(), KeyStoreError> {
        let mut store = self.store.lock().await;

        match store.contains_key(&key) {
            true => {
                let curr_val = store.entry(key).or_insert(V::default());
                *curr_val = value;
            }
            false => {
                store.insert(key, value);
            }
        }
        return Ok(());
    }
}

pub enum KeyStoreError {
    ValueNotPresent,
}
