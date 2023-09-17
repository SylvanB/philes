use std::collections::HashMap;
use std::fs::{File, remove_file};
use std::hash::Hash;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};
use actix_multipart::Field;
use actix_web::web;
use nanoid::nanoid;
use crate::keystore::{KeyStore, KeyStoreError};
use futures::StreamExt;


pub(crate) fn get_file_by_id(filepath: &PathBuf) -> Box<Path> {
    todo!()
}

pub(crate) async fn save_file_to_disk<TKvStore, K, V>(kv_store: &TKvStore, added_files: &mut HashMap<K, V>, field: &mut Field) -> Result<(), FileError>
    where
        TKvStore: KeyStore<K, V>,
        K: Clone + Eq + Hash + Send + Sync+ From<String>,
        V: Clone + Default + Send + Sync+ From<String>,
{
    let content_type = field.content_disposition().clone();
    let filename = content_type.get_filename().unwrap();
    let filepath = PathBuf::from(format!("/tmp/philes/{}", &filename));
    let id = nanoid!();

    // Create the file
    let mut f = File::create(&filepath).unwrap();

    // Write the chunks to disk
    while let Some(chunk) = field.next().await {
        let data = chunk.unwrap();
        f = match web::block(move || f.write_all(&data).map(|_| f)).await {
            Ok(f) => f.unwrap(),
            Err(_) => return Err(FileError::FailedToWriteToDisk)
        };
    }

    // Update the KeyStore
    match kv_store
        .upsert(id.clone().into(), format!("/tmp/philes/{}", filename).into())
        .await
    {
        Ok(_) => {
            added_files.insert(id.into(), format!("/tmp/philes/{}", &filename).into());
            Ok(())
        }
        Err(err) => {
            // We failed to update the KeyStore, we need to delete the file we just wrote to disk
            if let Err(err) = delete_file_from_disk(&filepath).await {
                return Err(err)
            } else {
                Err(FileError::KvStoreError(err))
            }
        },
    }
}

pub(crate) async fn delete_file_from_disk(filepath: &PathBuf) -> Result<(), FileError> {
    match remove_file(filepath) {
        Ok(_) => Ok(()),
        Err(err) => Err(FileError::IoError(err))
    }
}

pub enum FileError {
    FailedToWriteToDisk,
    KvStoreError(KeyStoreError),
    IoError(Error)
}