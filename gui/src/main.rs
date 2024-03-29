mod ca1dspace;
// mod pop_up;

use iced::{Alignment, executor, Length};
use iced::{Application, Command, Element, Settings, Theme};
use iced::widget::{button, column, container};
use iced::theme::Button;

pub fn main() -> iced::Result {
    CA1DSpace::run(Settings::default())
}

#[derive(Default)]
struct CA1DSpace;
// {
//     rule_name: String,
// }

#[derive(Debug, Clone, Copy)]
enum Message {
    Exit,
}

impl Application for CA1DSpace {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self::default(),
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("MonoAxis")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Exit => {
                std::process::exit(0);
            }

            // Message::Exit => window::close(iced::window::Id),
            // _ => Command::none()
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let content = column![
            "Are you sure you want to exit?",
            button("Yes, exit now")
            .style(Button::Destructive)
            .padding([10, 20])
            .on_press(Message::Exit),
            ]
            .spacing(10)
            .align_items(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
        // Theme::default()
    }
}
