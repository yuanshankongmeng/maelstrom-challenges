use crate::command_service::CommandService;
use crate::message::{EchoOk, InitOk, Message, Payload};

pub struct Node {}

impl CommandService for Node {
    fn execute(&self, request: Message) -> Result<Message, String> {
        match request.body {
            Some(Payload::Init(init)) => Ok(Message {
                src: request.dest,
                dest: request.src,
                body: Some(Payload::InitOk(InitOk {
                    in_reply_to: init.msg_id,
                })),
            }),
            Some(Payload::Echo(echo)) => Ok(Message {
                src: request.dest,
                dest: request.src,
                body: Some(Payload::EchoOk(EchoOk {
                    in_reply_to: echo.msg_id,
                    msg_id: echo.msg_id,
                    echo: echo.echo,
                })),
            }),
            _ => Err("body type not supported".to_string()),
        }
    }
}
