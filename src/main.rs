use iced::widget::{
    button, checkbox, column, container, row, text, text_editor, text_input, Column,
};
use iced::{Alignment, Element, Length, Renderer, Sandbox, Settings, Theme};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
pub fn main() -> iced::Result {
    // let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let pool = match PgPoolOptions::new()
    //     .max_connections(10)
    //     .connect(&database_url)
    //     .unwrap()
    // {
    //     Ok(pool) => {
    //         println!("✅Connection to the database is successful!");
    //         pool
    //     }
    //     Err(err) => {
    //         println!("🔥 Failed to connect to the database: {:?}", err);
    //         std::process::exit(1);
    //     }
    // };
    Counter::run(Settings::default())
}
struct Counter {
    num: i32,
    checkbox_value: bool,
    text: String,
}
#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
    Decriment,
    CheckboxToggled(bool),
    InputChanged(String),
}
impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self {
            num: 0,
            checkbox_value: false,
            text: String::from(""),
        }
    }

    fn title(&self) -> String {
        "Hello_postgresql".to_string()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Decriment => self.num -= 1,
            Message::ButtonPressed => {
                println!("{}", self.text);
                self.text = "".to_string()
            }
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::InputChanged(text) => {
                // println!("{}", text);
                self.text = text;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let button = button("Subbmit")
            .padding(10)
            .on_press(Message::ButtonPressed);
        let textinput = text_input("Type something...", &self.text)
            .on_input(Message::InputChanged)
            .padding(10)
            .size(20);


        let content = column![row![textinput, button]
            .spacing(10)
            .align_items(Alignment::Center),];
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
