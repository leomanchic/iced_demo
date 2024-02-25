use iced::alignment::{Horizontal, Vertical};
use iced::widget::{
    button, checkbox, column, container, horizontal_space, pick_list, row, text, text_input, Column,
};
use iced::{
    theme, Alignment, Application, Color, Command, Element, Length, Renderer, Settings, Theme,
};

// mod sqlreq;

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

#[derive(Debug, Clone)]

pub enum Choices {
    Choice1,
    Choice2,
    Choice3,
    Choice4,
}
pub struct MyApp {
    theme: Theme,
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
    ThemeChanged(Theme),
}
impl Application for MyApp {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                num: 0,
                checkbox_value: false,
                text: String::new(),
                theme: Theme::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "PostgresqlClient".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::Decriment => self.num -= 1,
            Message::ButtonPressed => {
                println!("{}", self.text);
                // sqlreq::make(&self.text).unwrap();
                self.text = String::new()
            }
            Message::CheckboxToggled(value) => self.checkbox_value = value,
            Message::InputChanged(text) => {
                // println!("{}", text);
                self.text = text;
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let choose_theme = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);

        let button = button("Subbmit").on_press(Message::ButtonPressed);
        let textinput = text_input("Type something...", &self.text)
            .on_input(Message::InputChanged)
            .on_submit(Message::ButtonPressed);

        let status_bar = row![
            horizontal_space(),
            choose_theme.padding(5),
            button.padding(5)
        ]
        .spacing(5);

        let content = container(
            column![horizontal_space(), textinput.padding(15), status_bar]
                .spacing(20)
                .align_items(Alignment::Center),
        )
        .into();
        content
    }
    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}
