use crate::message::Message;

pub trait CommandService {
    fn execute(&self, request: Message) -> Result<Message, String>;
}
