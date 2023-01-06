use anyhow::Error;
use deno_core::op;

#[op]
pub async fn ping() -> Result<String, Error> {
    Ok("pong".into())
}

#[op]
pub fn sum(numbers: Vec<f64>) -> Result<f64, Error> {
    Ok(numbers.iter().fold(0.0, |a, v| a + v))
}

#[op]
pub fn echo(text: String) -> String {
    text
}
