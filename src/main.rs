use iced::alignment::{Horizontal, Vertical};
use iced::application::Appearance;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, horizontal_space, pick_list, row, text,
    text_input, Column,
};
use iced::{
    theme, Alignment, Application, Color, Command, Element, Length, Renderer, Settings, Theme,
};

// mod sqlreq;

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

pub struct Menu {
    theme: Theme,
    text: String,
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
    MenuPressed,
}
impl Application for MyApp {
    type Message = Message;
    type Executor = iced::executor::Default;
    type Theme = Theme;
    type Flags = ();

    fn title(&self) -> String {
        "PostgresqlClient".to_string()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

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
            Message::MenuPressed => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let choose_theme = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);

        let buttone = button("Subbmit").on_press(Message::ButtonPressed);
        let button1 = button("menu").on_press(Message::MenuPressed);
        let button2 = button("config").on_press(Message::ButtonPressed);
        let button3 = button("clients").on_press(Message::ButtonPressed);
        let textinput = text_input("Type something...", &self.text)
            .on_input(Message::InputChanged)
            .on_submit(Message::ButtonPressed);

        let status_bar = row![
            choose_theme.padding(5),
            button1.padding(5),
            button2.padding(5),
            button3.padding(5),
            buttone.padding(5)
        ]
        .spacing(2);

        let content = container(
            column![
                horizontal_space(),
                status_bar.align_items(Alignment::Start),
                textinput.padding(10),
            ]
            .spacing(10),
        )
        .into();
        content
    }
}
