use glob::glob;
use iced::font::Weight;
use iced::widget::scrollable;
use iced::{widget, Element, Font, Length, Task};
use std::fmt::{Display, Formatter};
use std::fs::read;

mod font_manager;

fn font_loader() -> Vec<Vec<u8>> {
    match glob("./*.ttf") {
        Ok(v) => v
            .filter_map(|v| v.ok().and_then(|v| read(v).ok()))
            .collect(),
        Err(_) => Default::default(),
    }
}

fn main() -> iced::Result {
    let fonts = font_loader();
    let mut app = iced::application(FontViewer::new, FontViewer::update, FontViewer::view);
    for f in fonts {
        app = app.font(f);
    }
    app.run()
}

#[derive(Clone)]
enum Message {
    TextChanged(String),
    WeightChanged(FVWeight),
    TextSizeChanged(f32),
    FontFamilyFilterChanged(String),
}

#[derive(Clone)]
enum FVWeight {
    Thin,
    ExtraLight,
    Light,
    Normal,
    Medium,
    Semibold,
    Bold,
    ExtraBold,
    Black,
}

impl Default for FVWeight {
    fn default() -> Self {
        Weight::default().into()
    }
}

impl From<Weight> for FVWeight {
    fn from(value: Weight) -> Self {
        match value {
            Weight::Thin => FVWeight::Thin,
            Weight::ExtraLight => FVWeight::ExtraLight,
            Weight::Light => FVWeight::Light,
            Weight::Normal => FVWeight::Normal,
            Weight::Medium => FVWeight::Medium,
            Weight::Semibold => FVWeight::Semibold,
            Weight::Bold => FVWeight::Bold,
            Weight::ExtraBold => FVWeight::ExtraBold,
            Weight::Black => FVWeight::Black,
        }
    }
}

impl From<FVWeight> for Weight {
    fn from(value: FVWeight) -> Self {
        match value {
            FVWeight::Thin => Weight::Thin,
            FVWeight::ExtraLight => Weight::ExtraLight,
            FVWeight::Light => Weight::Light,
            FVWeight::Normal => Weight::Normal,
            FVWeight::Medium => Weight::Medium,
            FVWeight::Semibold => Weight::Semibold,
            FVWeight::Bold => Weight::Bold,
            FVWeight::ExtraBold => Weight::ExtraBold,
            FVWeight::Black => Weight::Black,
        }
    }
}

impl Display for FVWeight {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FVWeight::Thin => "Thin",
                FVWeight::ExtraLight => "ExtraLight",
                FVWeight::Light => "Light",
                FVWeight::Normal => "Normal",
                FVWeight::Medium => "Medium",
                FVWeight::Semibold => "Semibold",
                FVWeight::Bold => "Bold",
                FVWeight::ExtraBold => "ExtraBold",
                FVWeight::Black => "Black",
            }
        )
    }
}

struct FontViewer {
    text: String,
    weight: FVWeight,
    cb_weight: widget::combo_box::State<FVWeight>,
    text_size: f32,
    font_family_filter: String,
}

impl FontViewer {
    pub fn new() -> Self {
        Self {
            text: "ABC123".into(),
            weight: Default::default(),
            cb_weight: widget::combo_box::State::new(vec![
                FVWeight::Thin,
                FVWeight::ExtraLight,
                FVWeight::Light,
                FVWeight::Normal,
                FVWeight::Medium,
                FVWeight::Semibold,
                FVWeight::Bold,
                FVWeight::ExtraBold,
                FVWeight::Black,
            ]),
            text_size: 12f32,
            font_family_filter: Default::default(),
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TextChanged(v) => {
                self.text = v;
            }
            Message::WeightChanged(w) => {
                self.weight = w;
            }
            Message::TextSizeChanged(s) => {
                self.text_size = s;
            }
            Message::FontFamilyFilterChanged(f) => {
                self.font_family_filter = f;
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        fn th(text: &'_ str) -> widget::Text<'_> {
            widget::text(text).font(Font {
                weight: Weight::Bold,
                ..Default::default()
            })
        }

        iced::widget::column![
            widget::row![
                "Text",
                widget::text_input("please type a text.", self.text.as_str())
                    .on_input(Message::TextChanged)
                    .width(Length::Fill)
            ],
            widget::row![
                "Weight:",
                widget::combo_box(
                    &self.cb_weight,
                    "weight",
                    Some(&self.weight),
                    Message::WeightChanged
                )
            ],
            widget::row![
                "Size:",
                widget::slider(10f32..=200f32, self.text_size, Message::TextSizeChanged)
            ],
            widget::row![
                "Font Family Filter:",
                widget::text_input("font family", &self.font_family_filter)
                    .on_input(Message::FontFamilyFilterChanged)
            ],
            scrollable(iced::widget::table(
                vec![
                    widget::table::column(th("Font Family"), |font_family: &String| {
                        widget::text(font_family)
                    }),
                    widget::table::column(th("Text"), |font_family: &String| {
                        widget::text(self.text.as_str())
                            .size(self.text_size)
                            .font(Font {
                                weight: self.weight.clone().into(),
                                ..Font::with_name(font_family)
                            })
                    })
                    .width(Length::Fill),
                ],
                font_manager::get_global_font_list()
                    .iter()
                    .filter(|v| self.font_family_filter.is_empty()
                        || v.find(self.font_family_filter.as_str()).is_some()),
            ))
            .width(Length::Fill)
            .height(Length::Fill)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .padding(10)
        .into()
    }
}
