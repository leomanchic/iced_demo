use iced::{
    widget::{button, row, Container, Text},
    Element, Length,
};

use crate::Message;
use crate::Views;
pub struct SecondPage;

impl SecondPage {
    pub fn new() -> Self {
        SecondPage
    }

    pub fn view(&self) -> Element<Message> {
        let button = button("Back").on_press(Message::PageChanged(Views::Main));
        Container::new(row![Text::new("Hello from Page 2"), button].spacing(10))
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
