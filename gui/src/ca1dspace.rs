
// use iced::widget::canvas;
// use iced::Application;
// use iced::executor;
// use iced::theme::{self, Theme};
// use iced::Settings;
// use iced::Command;
// use iced::Element;
//
//
//
//
// #[derive(Default)]
// pub struct CA1DSpace {
//     pub cells: Vec<Vec<u8>>,
//     pub width: u32,
//     pub height: u32,
//     pub cell_size: u32,
//     pub rule: u8,
//     pub generation: u32,
//     pub generation_limit: u32,
//     pub paused: bool,
//     pub running: bool,
//     // pub canvas: canvas::State,
// }
//
// #[derive(Debug, Clone)]
// enum Message {
//     Exit,
// }
//
// impl Application for CA1DSpace {
//     type Message = Message;
//     type Theme = Theme;
//     type Executor = executor::Default;
//     type Flags = ();
//
//     fn new(_flags: ()) -> (Self, Command<Message>) {
//         (
//             Self {
//                 ..Self::default()
//             },
//             Command::none(),
//         )
//     }
//
//     fn view(&self) -> Element<Self::Message> {
//         "Hello, world!".into()
//     }
//     fn title(&self) -> String {
//         String::from("MonoAxis - Iced")
//     }
//
//     fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
//         match _message {
//             Message::Exit => {
//                 Command::perform(async {}, Message::Exit)
//             }
//         }
//         Command::none()
//     }
//
//
//     fn theme(&self) -> Self::Theme {
//         Theme::Dark
//     }
// }