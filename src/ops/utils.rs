use anyhow::Error;
use deno_core::op;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

#[op]
pub fn info(text: String) {
    log::info!("{text}");
}

#[op]
pub fn debug(text: String) {
    log::debug!("{text}");
}

#[op]
pub fn warn(text: String) {
    log::warn!("{text}");
}

#[op]
pub fn error(text: String) {
    log::error!("{text}");
}

#[op]
pub async fn read_input() -> Result<String, Error> {
    let mut reader = BufReader::new(stdin());
    let mut buffer = Vec::new();

    let _ = reader.read_until(b'\n', &mut buffer).await?;

    if let Some(last_char) = buffer.last() {
        if *last_char == b'\n' {
            buffer.pop();
        }
    }

    String::from_utf8(buffer).map_err(anyhow::Error::from)
}
