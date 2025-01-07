use crate::{
    storage::{HotReloadStorageItem, StorageItemFetcher},
    Error,
};

pub struct JSONFileHostReloadStorage {
    filename: String,
    file: Option<std::fs::File>,
    // file: Arc<RwLock<Option<std::fs::File>>>,
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

impl JSONFileHostReloadStorage {
    pub fn new(filename: String) -> Self {
        Self {
            filename,
            file: None,
            // file: Arc::new(RwLock::new(None)),
        }
    }
}

impl std::io::Read for JSONFileHostReloadStorage {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.file.is_none() {
            self.file = Some(std::fs::File::open(&self.filename)?);
        }
        self.file.as_mut().unwrap().read(buf)
        // let mut file = self.file.write().unwrap();
        // if file.is_none() {
        //     *file = Some(std::fs::File::open(&self.filename)?);
        // }
        // file.as_mut().unwrap().read(buf)
    }
}

impl std::io::Write for JSONFileHostReloadStorage {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        // let mut file = self.file.write().unwrap();
        // if file.is_none() {
        //     *file = Some(std::fs::File::create(&self.filename)?);
        // }
        // file.as_mut().unwrap().write(buf)
        if self.file.is_none() {
            self.file = Some(std::fs::File::create(&self.filename)?);
        }
        self.file.as_mut().unwrap().write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        // let mut file = self.file.write().unwrap();
        // if file.is_none() {
        //     *file = Some(std::fs::File::create(&self.filename)?);
        // }
        // file.as_mut().unwrap().flush()
        if self.file.is_none() {
            self.file = Some(std::fs::File::create(&self.filename)?);
        }
        self.file.as_mut().unwrap().flush()
    }
}

impl StorageItemFetcher for JSONFileHostReloadStorage {
    async fn fetch(&mut self) -> Result<HotReloadStorageItem, Error> {
        serde_json::from_reader(self).map_err(|e| Error::FetchStorage(e.to_string()))
    }
}
