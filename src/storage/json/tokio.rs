use log::debug;
use serde::Serialize;
use tokio::{
    fs::{File, OpenOptions},
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    storage::{HotReloadStorageItem, StorageItemFetcher},
    Error,
};

pub struct JSONFileHostReloadStorage {
    filename: String,
    file: Option<File>,
}

unsafe impl Sync for JSONFileHostReloadStorage {}
unsafe impl Send for JSONFileHostReloadStorage {}

impl Default for JSONFileHostReloadStorage {
    fn default() -> Self {
        Self {
            filename: "storage.json".to_string(),
            file: None,
        }
    }
}

impl StorageItemFetcher for JSONFileHostReloadStorage {
    async fn dump<T: Serialize>(&mut self, data: T) -> Result<(), Error> {
        debug!("JSONFileHostReloadStorage::dump");
        if self.file.is_none() {
            let file = OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .read(true)
                .open(&self.filename)
                .await
                .map_err(|e| Error::OpenFile(format!("打开文件{}失败: {e}", self.filename)))?;

            self.file = Some(file);
        }

        let buf = serde_json::to_vec(&data)?;

        let file = self.file.as_mut().unwrap();

        file.write_all(&buf)
            .await
            .map_err(|e| Error::OpenFile(format!("写入文件失败: {e}")))?;

        file.sync_data()
            .await
            .map_err(|e| Error::OpenFile(format!("同步文件数据失败: {e}")))?;

        Ok(())
    }

    async fn fetch(&mut self) -> Result<HotReloadStorageItem, Error> {
        if self.file.is_none() {
            let file = OpenOptions::new()
                .create(true)
                .truncate(false)
                .write(true)
                .read(true)
                .open(&self.filename)
                .await
                .map_err(|e| Error::OpenFile(format!("打开文件{}失败: {e}", self.filename)))?;
            self.file = Some(file);
        }

        let mut buf = Vec::new();
        self.file
            .as_mut()
            .unwrap()
            .read_to_end(&mut buf)
            .await
            .map_err(|e| Error::OpenFile(format!("读取文件失败: {e}")))?;

        Ok(serde_json::from_slice(&buf)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dump() {
        let json_str = r#"{"cookies":{"https://wx2.qq.com/cgi-bin/mmwebwx-bin/webwxstatusnotify":"","https://wx2.qq.com/cgi-bin/mmwebwx-bin/webwxinit":"","https://login.wx.qq.com/cgi-bin/mmwebwx-bin/login":"","https://wx2.qq.com/cgi-bin/mmwebwx-bin/webwxnewloginpage":"{\"raw_cookie\":\"Expires=Fri, 05-Jan-2035 06:18:22 GMT\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n{\"raw_cookie\":\"webwx_auth_ticket=CIsBEMLbgagKGoABoPfuX4SSdHsh4DC5Rdw37msyozVfjAMBvYT/pTuSlEDUuBxco1Z7ayZA3gdmCb0R40rUJAQgQk6Ay0lmHfxTFT3kER6AZBvrOkzisSTWnMw8MpiAtacpVSOQeEMN+Z4j5TrclsCFqX5e68jWT2zN2f9G8IVWFAb6mc+5ssCxJzc=\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n{\"raw_cookie\":\"wxsid=yCYetV96I2j/88wO\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n{\"raw_cookie\":\"Domain=wx2.qq.com\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n{\"raw_cookie\":\"webwx_data_ticket=gSe8IBRv4xqULn2LNy4M9x4L\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n{\"raw_cookie\":\"mm_lang=zh_CN\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n{\"raw_cookie\":\"webwxuvid=26a1cb79c1c0c1bfbc1912994c124abc6465d9e52f589b54cbe7215b848629a5bc70398fda991fa3281ca5523741211b\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n{\"raw_cookie\":\"wxloadtime=1736230702\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n{\"raw_cookie\":\"wxuin=2850172843\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n{\"raw_cookie\":\"Path=/\",\"path\":[\"/cgi-bin/mmwebwx-bin\",false],\"domain\":{\"HostOnly\":\"wx2.qq.com\"},\"expires\":\"SessionEnd\"}\n","https://login.wx.qq.com/jslogin":""},"base_request":{"Uin":2850172843,"Sid":"yCYetV96I2j/88wO","Skey":"@crypt_d7ac068f_100b9bea039ffda56dda4d47fcfd6a8d","DeviceID":"e354261774648813"},"login_info":{"ret":0,"wxuin":2850172843,"isgrayscale":1,"message":"","skey":"@crypt_d7ac068f_100b9bea039ffda56dda4d47fcfd6a8d","wxsid":"yCYetV96I2j/88wO","pass_ticket":"fDEumsZLaSwaFlPnBVY%2FSbIaTWT8SsWAwchbYGoTX9mdNQcvLyxGBkQzFxc3UGMsEGC3PbDtHEhyxm9If5rbqA%3D%3D"},"wechat_domain":"wx2.qq.com","uuid":"ob1vmlKrwA=="}"#;
        let items: HotReloadStorageItem = serde_json::from_str(json_str).unwrap();
        let mut storage = JSONFileHostReloadStorage::default();
        storage.dump(items).await.unwrap();
    }
}
