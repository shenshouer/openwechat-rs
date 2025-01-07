use serde::Serialize;
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    storage::{HotReloadStorageItem, StorageItemFetcher},
    Error,
};

pub struct JSONFileHostReloadStorage {
    filename: String,
    // file: Arc<RwLock<Option<File>>>,
    file: Option<File>,
}

unsafe impl Sync for JSONFileHostReloadStorage {}
unsafe impl Send for JSONFileHostReloadStorage {}

impl Default for JSONFileHostReloadStorage {
    fn default() -> Self {
        Self {
            filename: "storage.json".to_string(),
            file: None,
            // file: Arc::new(RwLock::new(None)),
        }
    }
}

impl StorageItemFetcher for JSONFileHostReloadStorage {
    async fn dump<T: Serialize>(&mut self, data: T) -> Result<(), Error> {
        if self.file.is_none() {
            let file = File::open(&self.filename)
                .await
                .map_err(|e| Error::OpenFile(format!("打开文件{}失败: {e}", self.filename)))?;
            self.file = Some(file);
        }

        let buf = serde_json::to_vec(&data)?;
        self.file
            .as_mut()
            .unwrap()
            .write_all(&buf)
            .await
            .map_err(|e| Error::OpenFile(format!("写入文件失败: {e}")))?;
        // let mut file_lock = self.file.write().await;
        // if file_lock.is_none() {
        //     let file = File::open(&self.filename)
        //         .await
        //         .map_err(|e| Error::OpenFile(format!("打开文件{}失败: {e}", self.filename)))?;
        //     *file_lock = Some(file);
        // }

        // let buf = serde_json::to_vec(&data)?;
        // (*file_lock)
        //     .as_mut()
        //     .unwrap()
        //     .write_all(&buf)
        //     .await
        //     .map_err(|e| Error::OpenFile(format!("写入文件失败: {e}")))?;

        Ok(())
    }

    async fn fetch(&mut self) -> Result<HotReloadStorageItem, Error> {
        let mut buf = Vec::new();
        if self.file.is_none() {
            let file = File::open(&self.filename)
                .await
                .map_err(|e| Error::OpenFile(format!("打开文件{}失败: {e}", self.filename)))?;
            self.file = Some(file);
        }
        self.file
            .as_mut()
            .unwrap()
            .read_to_end(&mut buf)
            .await
            .map_err(|e| Error::OpenFile(format!("读取文件失败: {e}")))?;

        // let mut file_lock = self.file.write().await;
        // if file_lock.is_none() {
        //     let file = File::open(&self.filename)
        //         .await
        //         .map_err(|e| Error::OpenFile(format!("打开文件{}失败: {e}", self.filename)))?;
        //     *file_lock = Some(file);
        // }

        // (*file_lock)
        //     .as_mut()
        //     .unwrap()
        //     .read_to_end(&mut buf)
        //     .await
        //     .map_err(|e| Error::OpenFile(format!("读取文件失败: {e}")))?;

        Ok(serde_json::from_slice(&buf)?)
    }
}
