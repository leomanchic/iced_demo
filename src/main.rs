use iced::alignment::{Horizontal, Vertical};
use iced::application::Appearance;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, horizontal_space, pick_list, row, text,
    text_input, Column, Container,
};
use iced::{
    theme, Alignment, Application, Color, Command, Element, Length, Renderer, Settings, Theme,
};
use pagetest::SecondPage;

mod pagetest;

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

pub struct MyApp {
    theme: Theme,
    text: String,
    page: SecondPage,
    current_view: Views,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
    InputChanged(String),
    ThemeChanged(Theme),
    PageChanged(Views),
}

#[derive(Debug, Clone)]
pub enum Views {
    Main,
    Second,
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
                page: SecondPage::new(),
                text: String::new(),
                theme: Theme::default(),
                current_view: Views::Main,
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::ButtonPressed => {
                println!("{}", self.text);
                // sqlreq::make(&self.text).unwrap();
                self.text = String::new()
            }
            Message::InputChanged(text) => {
                // println!("{}", text);
                self.text = text;
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
            }
            Message::PageChanged(value) => self.current_view = value,
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let choose_theme = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);

        let buttone = button("Subbmit").on_press(Message::ButtonPressed);
        let button1 = button("menu").on_press(Message::PageChanged(Views::Second));
        let button2 = button("config").on_press(Message::ButtonPressed);
        let button3 = button("clients").on_press(Message::ButtonPressed);
        let textinput = text_input("Type something...", &self.text)
            // .line_height(text::LineHeight::Relative(1.75))
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

        let content = Container::new(
            column![
                horizontal_space(),
                status_bar.align_items(Alignment::Start),
                textinput.padding(10),
            ]
            .spacing(10),
        );
        match self.current_view {
            Views::Main => content.into(),
            Views::Second => self.page.view(),
        }
    }
}
