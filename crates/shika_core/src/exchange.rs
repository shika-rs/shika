use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::io::Cursor;
use anyhow::anyhow;
use hyper::Method;
use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt};
use Method::Get;

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>
}

impl<R> From<hyper::Request<R>> for Request {
    fn from(value: hyper::Request<R>) -> Self {
        let method: Method = value.method().try_into()?;


        let path: String = value.uri().path().to_string();

        // Insert headers.
        let mut headers = HashMap::new();
        value.headers().iter().for_each(|(key, value)| {
            headers.insert(
                key.to_string(),
                value.to_string()
            )
        });

        Request {
            method,
            path,
            headers
        }
    }
}

pub enum Status {
    Ok,
    NotFound
}

impl Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Ok => write!(f, "200 OK"),
            Status::NotFound => write!(f, "404 Not Found"),
        }
    }
}




pub struct Response<S: AsyncRead + Unpin> {
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub data: S
}

impl Response<Cursor<Vec<u8>>> {
    pub fn from_html(status: Status, data: impl ToString) -> Self {
        let bytes = data.to_string().into_bytes();

        let mut headers = HashMap::new();

        headers.insert("Content-Type".to_string(), "text/html".to_string());
        headers.insert("Content-Length".to_string(), bytes.len().to_string());

        Self {
            status,
            headers,
            data: Cursor::new(bytes),
        }
    }
}

impl <S: AsyncRead + Unpin> Response<S> {
    pub fn status_and_headers(&self) -> String {
        let headers = self
            .headers
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect::<Vec<_>>()
            .join("\r\n");

        format!("HTTP/1.1 {}\r\n{headers}\r\n\r\n", self.status)
    }

    pub async fn write<O: AsyncWrite + Unpin>(mut self, stream: &mut O) -> anyhow::Result<()> {
        stream
            .write_all(self.status_and_headers().as_bytes())
            .await?;

        tokio::io::copy(&mut self.data, stream).await?;

        Ok(())
    }
}