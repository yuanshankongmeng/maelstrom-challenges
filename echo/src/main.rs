mod command_service;
mod message;
mod node;

use std::io::{stdin, stdout, Write};

use anyhow::Context;
use command_service::*;
use message::*;
use node::*;
use serde_json::Deserializer;

fn main() -> anyhow::Result<()> {
    let stdin = stdin().lock();
    let mut stdout = stdout().lock();

    let node = Node {};

    let inputs = Deserializer::from_reader(stdin).into_iter::<Message>();
    for input in inputs {
        let input = input.context("input can not be deserialized")?;
        if let Ok(response) = node.execute(input) {
            let response_str = serde_json::to_string(&response)?;
            stdout.write(response_str.as_bytes())?;
            stdout.write(b"\n").context("write trailing newline")?;
            stdout.flush()?;
        }
    }
    Ok(())
}
