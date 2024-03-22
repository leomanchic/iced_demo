use iced::alignment::{Horizontal, Vertical};
use iced::application::Appearance;
use iced::widget::shader::wgpu::naga::valid;
use iced::widget::{
    button, checkbox, column, container, horizontal_rule, horizontal_space, pick_list, radio, row,
    text, text_editor, text_input, toggler, Column, Container,
};
use iced::{
    theme, Alignment, Application, Color, Command, Element, Length, Renderer, Settings, Theme,
};
mod gui;
use gui::pagetest::SecondPage;
// mod tablebuilder;
mod sqlxmod;

pub fn main() -> iced::Result {
    MyApp::run(Settings::default())
}

pub struct MyApp {
    theme: Theme,
    text: String,
    page: SecondPage,
    current_view: Views,
    content: text_editor::Content,
    command_to_pg: Choice,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Choice {
    Execute,
    Querry,
}

#[derive(Debug, Clone)]
pub enum Message {
    ButtonPressed,
    InputChanged(String),
    ThemeChanged(Theme),
    PageChanged(Views),
    Edit(text_editor::Action),
    RadioSelected(Choice),
    
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
                content: text_editor::Content::new(),
                command_to_pg: Choice::Execute,
            },
            Command::none(),
        )
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::ButtonPressed => {
                println!("{}", self.text);
                sqlxmod::process(self.text.clone(), &self.command_to_pg).unwrap();
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
            Message::Edit(action) => {
                self.content.perform(action);
            }
            Message::RadioSelected(Choice::Execute) => {
                self.command_to_pg = Choice::Execute;
            }
            Message::RadioSelected(Choice::Querry) => {
                self.command_to_pg = Choice::Querry;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let choose_theme = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);

        let buttone = button("Subbmit").on_press(Message::ButtonPressed);
        let pager = button("menu").on_press(Message::PageChanged(Views::Second));
    

        let radio_but_exec = radio(
            "exequte",
            Choice::Execute,
            Some(self.command_to_pg),
            Message::RadioSelected,
        );
        let radio_but_querry = radio(
            "querry",
            Choice::Querry,
            Some(self.command_to_pg),
            Message::RadioSelected,
        );
        let textinput = text_input("Напиши свой запрос...", &self.text)
            // .line_height(text::LineHeight::Relative(1.75))
            .on_input(Message::InputChanged)
            .on_submit(Message::ButtonPressed);
        let row = row![radio_but_exec, radio_but_querry]
            .padding(10)
            .spacing(10);
        let querry_builder = text_editor(&self.content).on_action(Message::Edit);
        let position = {
            let (line, column) = self.content.cursor_position();
            text(format!("{}:{}", line, column))
        };

        let status_bar = row![choose_theme.padding(5), buttone.padding(5), pager.padding(5)].spacing(2);

        let content = Container::new(
            column![
                status_bar.align_items(Alignment::Start),
                textinput.padding(10),
                row,
                querry_builder.padding(10),
                position
                    .vertical_alignment(Vertical::Bottom)
                    .horizontal_alignment(Horizontal::Left),
            ]
            .spacing(10)
            .padding(10),
        );
        match self.current_view {
            Views::Main => content.into(),
            Views::Second => self.page.view(),
        }
    }
}
