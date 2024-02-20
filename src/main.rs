use iced::widget::{button, column, text, Column};
use iced::{Element, Renderer, Sandbox, Settings, Theme};
pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}
struct Counter {
    num: i32,
}
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Incriment,
    Decriment,
}
impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { num: 0 }
    }

    fn title(&self) -> String {
        "Hello_postgresql".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Decriment => self.num -= 1,
            Message::Incriment => self.num += 1,
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            button("+").on_press(Message::Incriment),
            text(self.num).size(50),
            button("-").on_press(Message::Decriment),
        ]
        .into()
    }
}
