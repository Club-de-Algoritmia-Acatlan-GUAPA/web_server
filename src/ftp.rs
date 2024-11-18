use anyhow::Result;
use futures::{io::AsyncReadExt, TryStreamExt};
use reqwest::{multipart, Body, Client};
use suppaftp::AsyncFtpStream;
use tokio::io::AsyncRead;
use tokio_util::codec::{BytesCodec, FramedRead};

#[derive(Clone)]
pub struct FTPClient {
    uri: String,
    client: Client,
}

impl FTPClient {
    pub fn new(uri: String) -> Self {
        Self {
            uri,
            client: Client::new(),
        }
    }

    pub async fn connect(&self) -> Result<AsyncFtpStream> {
        let mut con = AsyncFtpStream::connect(&self.uri).await?;
        con.login("", "").await?;
        Ok(con)
    }

    pub async fn mkdir(&self, name: &str) -> Result<()> {
        let url = format!("{}/dir/{}", self.uri, name);
        dbg!(&url);
        self.client.post(url).send().await?;
        Ok(())
    }

    pub async fn store_file(&self, filename: &str, directory: &str, file: Vec<u8>) -> Result<()> {
        let form = file_to_multipart(filename, file).await?;
        let url = format!("{}/file/{}", self.uri, directory);
        dbg!(&url);
        self.client.post(url).multipart(form).send().await?;

        Ok(())
    }

    pub async fn get_file(&self, directory: &str, filename: &str) -> Result<Vec<u8>> {
        let res = self
            .client
            .get(format!("{}/file/{}/{}", self.uri, directory, filename))
            .send()
            .await?;
        Ok(res.bytes().await?.to_vec())
    }

    pub async fn store_checker(&self, directory: &str, file: Vec<u8>) -> Result<()> {
        let form = file_to_multipart("checker.cpp", file).await?;
        let url = format!("{}/checker/{}", self.uri, directory);
        dbg!(&url);
        self.client.post(url).multipart(form).send().await?;
        Ok(())
    }

    pub async fn remove_file(&self, filename: &str) -> Result<()> {
        let mut con = self.connect().await?;
        match con.rm(filename).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}

async fn file_to_multipart(filename: &str, file: Vec<u8>) -> Result<multipart::Form> {
    let some_file = multipart::Part::stream(file)
        .file_name(filename.to_string())
        .mime_str("text/plain")?;

    let form = multipart::Form::new().part("file", some_file);
    Ok(form)
}
