use anyhow::Result;
use futures_util::AsyncRead;
use suppaftp::AsyncFtpStream;

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
        name: &mut R,
    ) -> Result<()> {
        let mut con = self.connect().await?;
        let _ = con.put_file(filename, name).await?;
        Ok(())
    }
}
