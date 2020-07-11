// レイアウト
use iced::Column;
use iced::Container;
use iced::Row;

// ウィジェット
use iced::button;
use iced::image;
use iced::Button;
use iced::Image;
use iced::Text;

// 全体
use iced::Element;
use iced::Sandbox;

#[derive(Debug)]
pub enum Pages {
    StartPage {
        show_button_state: button::State,
    },
    ImagePage {
        img: image::Handle,
        back_button_state: button::State,
    },
}
#[derive(Debug, Clone)]
pub enum Message {
    Show,
    Back,
}

impl Sandbox for Pages {
    type Message = Message;
    fn view(&mut self) -> Element<'_, Self::Message> {
        let content = match self {
            Pages::StartPage { show_button_state } => Container::new(
                Column::new()
                    .spacing(20)
                    .align_items(iced::Align::Center)
                    .push(Text::new("Push button below!"))
                    .push(
                        Button::new(show_button_state, Text::new("PLESS TO SHOW"))
                            .padding(10)
                            .on_press(Message::Show),
                    ),
            ),
            Pages::ImagePage {
                img,
                back_button_state,
            } => Container::new(
                Row::new()
                    .spacing(20)
                    .align_items(iced::Align::Center)
                    .push(Image::new(img.clone()))
                    .push(
                        Button::new(back_button_state, Text::new("PLESS TO BUCK"))
                            .padding(10)
                            .on_press(Message::Back),
                    ),
            ),
        };
        content
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
    fn new() -> Self {
        Pages::StartPage {
            show_button_state: button::State::default(),
        }
    }
    fn title(&self) -> String {
        "test app".to_string()
    }
    fn update(&mut self, message: Self::Message) {
        match message {
            Message::Show => {
                *self = Pages::ImagePage {
                    back_button_state: button::State::default(),
                    img: image::Handle::from_path("img/lena.png"),
                }
            }
            Message::Back => {
                *self = Pages::StartPage {
                    show_button_state: button::State::default(),
                }
            }
        }
    }
}
