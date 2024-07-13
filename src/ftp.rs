use anyhow::Result;
use futures::io::AsyncReadExt;
use futures_util::AsyncRead;
use suppaftp::{AsyncFtpStream, FtpResult};

#[derive(Clone)]
pub struct FTPClient {
    uri: String,
}

impl FTPClient {
    pub fn new(uri: String) -> Self {
        Self { uri }
    }

    pub async fn connect(&self) -> Result<AsyncFtpStream> {
        let mut con = AsyncFtpStream::connect(&self.uri).await?;
        con.login("", "").await?;
        Ok(con)
    }

    pub async fn mkdir(&self, name: &str) -> Result<()> {
        let mut con = self.connect().await?;
        Ok(con.mkdir(name).await?)
    }

    pub async fn store_file<R: AsyncRead + Unpin>(
        &self,
        filename: &str,
        file: &mut R,
    ) -> Result<()> {
        let mut con = self.connect().await?;
        let _ = con.put_file(filename, file).await?;
        Ok(())
    }

    pub async fn get_file_as_stream(&self, filename: &str) -> Result<Vec<u8>> {
        let mut con = self.connect().await?;
        match con.retr_as_stream(filename).await {
            Ok(mut stream) => {
                let mut buf: Vec<u8> = Vec::new();
                let _ = (&mut stream).read_to_end(&mut buf).await?;
                con.finalize_retr_stream(stream).await?;
                Ok(buf)
            },
            Err(err) => Err(err.into()),
        }
    }

    pub async fn remove_file(&self, filename: &str) -> Result<()> {
        let mut con = self.connect().await?;
        match con.rm(filename).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.into()),
        }
    }
}
