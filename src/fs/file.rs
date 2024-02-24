use rquickjs::{Ctx, Exception, Result as QuickJsResult};

use tokio::io::{
    AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader as AsyncBufReader,
    BufWriter as AsyncBufWriter,
};

use std::io::{BufRead, BufReader, BufWriter, Read, Write};

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct File {
    #[qjs(skip_trace)]
    inner: tokio::fs::File,
}

#[rquickjs::methods(rename_all = "camelCase")]
impl File {
    #[qjs(skip)]
    pub fn new(inner: tokio::fs::File) -> File {
        File { inner }
    }

    pub async fn read(&mut self, ctx: Ctx<'_>) -> QuickJsResult<String> {
        let mut reader = AsyncBufReader::new(&mut self.inner);

        let mut buf = String::new();

        match reader.read_to_string(&mut buf).await {
            Ok(_) => (),
            Err(err) => {
                return Err(Exception::throw_message(
                    &ctx,
                    &format!("Could not read file: {}", err),
                ))
            }
        };

        Ok(buf)
    }

    pub async fn read_line(&mut self, ctx: Ctx<'_>) -> QuickJsResult<String> {
        let mut reader = AsyncBufReader::new(&mut self.inner);

        let mut buf = String::new();

        match reader.read_line(&mut buf).await {
            Ok(_) => (),
            Err(err) => {
                return Err(Exception::throw_message(
                    &ctx,
                    &format!("Could not read line: {}", err),
                ))
            }
        };

        Ok(buf)
    }

    pub async fn write(&mut self, ctx: Ctx<'_>, buf: String) -> QuickJsResult<()> {
        let mut writer = AsyncBufWriter::new(&mut self.inner);

        match writer.write_all(buf.as_bytes()).await {
            Ok(_) => (),
            Err(err) => {
                return Err(Exception::throw_message(
                    &ctx,
                    &format!("Could not write to file: {}", err),
                ))
            }
        };

        match writer.flush().await {
            Ok(_) => (),
            Err(err) => {
                return Err(Exception::throw_message(
                    &ctx,
                    &format!("Could not write to file: {}", err),
                ))
            }
        };

        Ok(())
    }
}

#[rquickjs::class]
#[derive(rquickjs::class::Trace)]
pub struct FileSync {
    #[qjs(skip_trace)]
    inner: std::fs::File,
}

#[rquickjs::methods(rename_all = "camelCase")]
impl FileSync {
    #[qjs(skip)]
    pub fn new(inner: std::fs::File) -> FileSync {
        FileSync { inner }
    }

    pub fn read_sync(&self, ctx: Ctx<'_>) -> QuickJsResult<String> {
        let mut reader = BufReader::new(&self.inner);

        let mut buf = String::new();

        match reader.read_to_string(&mut buf) {
            Ok(_) => (),
            Err(err) => {
                return Err(Exception::throw_message(
                    &ctx,
                    &format!("Could not read file: {}", err),
                ))
            }
        };

        Ok(buf)
    }

    pub fn read_line_sync(&self, ctx: Ctx<'_>) -> QuickJsResult<String> {
        let mut reader = BufReader::new(&self.inner);

        let mut buf = String::new();

        match reader.read_line(&mut buf) {
            Ok(_) => (),
            Err(err) => {
                return Err(Exception::throw_message(
                    &ctx,
                    &format!("Could not read line: {}", err),
                ))
            }
        };

        Ok(buf)
    }

    pub fn write_sync(&self, ctx: Ctx<'_>, buf: String) -> QuickJsResult<()> {
        let mut writer = BufWriter::new(&self.inner);

        match writer.write_all(buf.as_bytes()) {
            Ok(_) => (),
            Err(err) => {
                return Err(Exception::throw_message(
                    &ctx,
                    &format!("Could not write to file: {}", err),
                ))
            }
        };

        match writer.flush() {
            Ok(_) => (),
            Err(err) => {
                return Err(Exception::throw_message(
                    &ctx,
                    &format!("Could not write to file: {}", err),
                ))
            }
        };

        Ok(())
    }
}
