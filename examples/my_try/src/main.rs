use iced::alignment::{Horizontal, Vertical};
// use iced::widget::scrollable;
use iced::widget::{column, row};
use iced::widget::{container, image, text, Button};
use iced::{alignment, Alignment, Background, Color, Length, Theme};
use iced::{Element, Sandbox, Settings};
// use iced_cored::{Appearance, StyleSheet};
use log::{info, log_enabled, Level};
use std::env::set_var;
pub fn main() -> iced::Result {
    set_var("RUST_LOG", "debug");
    env_logger::init();
    MainWin::run(Settings::default())
}

pub struct MainWin {
    procedure: Procedure,
    debug: bool,
}

impl Sandbox for MainWin {
    type Message = Message;
    fn new() -> MainWin {
        MainWin {
            procedure: Procedure::new(),
            debug: true,
        }
    }

    fn title(&self) -> String {
        format!("Procedure: {}", self.procedure.title())
    }

    fn view(&self) -> Element<Message> {
        // get content in this procedure
        let MainWin { procedure, .. } = self;
        // display: Row by Row
        let mut display_content = column![];
        // title
        // todo : theme ? message ?
        let main_win_title = format!("Process {}", procedure.current_index);
        display_content = display_content.push(
            text(&main_win_title)
                .width(Length::Fill)
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center)
                .height(Length::FillPortion(1)),
        );
        // task && result
        display_content = display_content.push(
            container(procedure.view(self.debug).map(Message::ProcessMsg))
                .width(Length::Fill)
                .height(Length::FillPortion(8)),
        );
        // controls && hint
        let display_content: Element<_> = display_content
            .push(
                row![
                    button(if procedure.has_previous() {
                        "Back"
                    } else {
                        "None"
                    }),
                    text("input audio wav display")
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center)
                        .width(Length::Fill),
                    button(if procedure.has_next() { "Next" } else { "None" }),
                ]
                .width(Length::Fill)
                .align_items(Alignment::Center)
                .height(Length::FillPortion(2))
                .spacing(20),
            )
            .spacing(20)
            .padding(20)
            .into();

        // let scrollable = scrollable(
        container(if self.debug {
            display_content.explain(Color::BLACK)
        } else {
            display_content
        })
        .width(Length::Fill)
        .center_x()
        // );
        .height(Length::Fill)
        .center_y()
        .into()
        // container(scrollable).height(Length::Fill).center_y().into()
    }

    fn update(&mut self, event: Message) {
        match event {
            Message::BackPressed => self.procedure.go_back(),
            Message::NextPressed => self.procedure.go_next(),
            Message::ProcessMsg(process_msg) => {
                self.procedure.update(process_msg, &mut self.debug);
            }
        }
    }
}

struct Procedure {
    processes: Vec<Process>,
    current_index: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    BackPressed,
    NextPressed,
    ProcessMsg(ProcessMessage),
}

impl Procedure {
    fn new() -> Procedure {
        Procedure {
            processes: vec![
                Process::Welcome {
                    image_path: format!(
                        "{}/images/ferris.png",
                        env!("CARGO_MANIFEST_DIR")
                    ),
                    hint: "this is a welcome",
                },
                Process::Finished {
                    hint: "this is a finish window!",
                },
            ],
            current_index: 0,
        }
    }

    fn view(&self, debug: bool) -> Element<ProcessMessage> {
        self.processes[self.current_index].view(debug)
        // display_content = display_content.push(row![
        //     container(image("tour/images/ferris.png"))
        //         .width(Length::FillPortion(0.7)),
        //         .center_x()
        //     container(column![text("Right"), text("Wrong")])
        //         .width(Length::FillPortion(0.3)),
        //         .center_x()
        // ]);
    }

    fn update(&mut self, msg: ProcessMessage, debug: &mut bool) {
        self.processes[self.current_index].update(msg, debug);
    }

    fn go_next(&mut self) {
        if self.has_next() {
            self.current_index += 1;
        }
    }
    fn go_back(&mut self) {
        if self.has_previous() {
            self.current_index -= 1;
        }
    }

    fn has_next(&self) -> bool {
        self.current_index + 1 < self.processes.len()
            && self.processes[self.current_index].can_continue()
    }
    fn has_previous(&self) -> bool {
        self.current_index > 0
    }
    fn title(&self) -> &str {
        self.processes[self.current_index].title()
    }
}

enum Process {
    Welcome {
        image_path: String,
        hint: &'static str,
    },
    Finished {
        hint: &'static str,
    },
}

impl Process {
    fn update(&mut self, msg: ProcessMessage, _debug: &mut bool) {
        match msg {
            ProcessMessage::Welcome { image_path, hint } => {
                if log_enabled!(Level::Info) {
                    info!("Process Welcome update: {} {}", image_path, hint);
                }
            }
            ProcessMessage::Finished { hint } => {
                if log_enabled!(Level::Info) {
                    info!("Process Finished update: {}", hint);
                }
            }
        }
    }
    fn title(&self) -> &str {
        match self {
            Process::Welcome { .. } => "Welcome",
            Process::Finished { .. } => "Finished",
        }
    }
    fn can_continue(&self) -> bool {
        true
    }
    fn view(&self, _debug: bool) -> Element<ProcessMessage> {
        if log_enabled!(Level::Info) {
            info!("process view");
        }
        let mut display_container = row![];
        match self {
            Process::Welcome { image_path, hint } => {
                if log_enabled!(Level::Info) {
                    info!("image path: {}", image_path);
                }
                display_container = display_container.push(
                    column![
                        container(image(image_path).width(Length::Fill))
                            .height(Length::FillPortion(8)),
                        container(
                            text(hint).horizontal_alignment(Horizontal::Center)
                        )
                        .style(MyContainerStyle::Debug.to_fn())
                    ]
                    .padding(20)
                    .align_items(Alignment::Center)
                    .width(Length::FillPortion(7)),
                )
            }
            Process::Finished { hint } => {
                display_container =
                    display_container.push(text(hint).height(Length::Fill))
            }
        };
        display_container
            .push(
                container(
                    column![
                        text("Right")
                            .horizontal_alignment(Horizontal::Center)
                            .height(Length::Fill)
                            .width(Length::Fill),
                        text("Wrong")
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .horizontal_alignment(Horizontal::Center)
                    ]
                    .align_items(Alignment::Center)
                    .padding(20),
                )
                .style(MyContainerStyle::Debug.to_fn())
                .width(Length::FillPortion(3)),
            )
            .padding(20)
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum ProcessMessage {
    Welcome {
        image_path: &'static str,
        hint: &'static str,
    },
    Finished {
        hint: &'static str,
    },
}

fn button<'a, Message: Clone>(label: &str) -> Button<'a, Message> {
    iced::widget::button(
        text(label).horizontal_alignment(alignment::Horizontal::Center),
    )
    .padding(12)
    .width(Length::Units(100))
}

pub mod my_color {
    use iced::Color;
    pub const BLUE: Color = Color {
        r: 0_f32,
        g: 0.0,
        b: 1.0,
        a: 1.0,
    };
    pub const RED: Color = Color {
        r: 1_f32,
        g: 0.0,
        b: 0.0,
        a: 1.0,
    };
    pub const GREEN: Color = Color {
        r: 0_f32,
        g: 1.0,
        b: 0.0,
        a: 1.0,
    };
}

pub enum MyContainerStyle {
    Debug,
}

impl MyContainerStyle {
    fn to_fn(&self) -> fn(&Theme) -> container::Appearance {
        match self {
            MyContainerStyle::Debug => |_| container::Appearance {
                text_color: Some(my_color::BLUE),
                background: Some(Background::Color(my_color::RED)),
                border_width: 5.0,
                border_color: my_color::GREEN,
                border_radius: 5.0,
            },
        }
    }
}

// impl Default for MyContainerStyle {
//     fn default() -> Self {
//         MyContainerStyle::debug
//     }
// }

// impl container::StyleSheet for MyContainerStyle {
//     type Style = Box<MyContainerStyle>;
//     fn appearance(&self, style: &Self::Style) -> container::Appearance {
//         match **style {
//             MyContainerStyle::debug => container::Appearance {
//                 text_color: Some(MyColor::BLUE),
//                 background: Some(Background::Color(MyColor::RED)),
//                 border_width: 5.0,
//                 border_color: MyColor::GREEN,
//                 border_radius: 5.0,
//             },
//         }
//     }
// }
