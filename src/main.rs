use crate::font_manager::get_global_font_list;
use glob::glob;
use iced::font::Weight;
use iced::widget::scrollable;
use iced::{widget, Element, Font, Length, Task};
use std::collections::HashSet;
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
    Pin(usize),
    HideUnpin(bool),
    UpdateShownFontFamily,
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
    font_families: &'static Vec<String>,
    pinned: Vec<usize>,
    font_family_filter_pattern: Vec<String>,
    hide_unpin: bool,
    shown_family: Vec<usize>,
}

impl FontViewer {
    pub fn new() -> (Self, Task<Message>) {
        (
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
                font_families: get_global_font_list(),
                pinned: vec![],
                font_family_filter_pattern: vec![],
                hide_unpin: false,
                shown_family: vec![],
            },
            Task::done(Message::UpdateShownFontFamily),
        )
    }

    pub fn compile_font_family_filter(&mut self) {
        self.font_family_filter_pattern = self
            .font_family_filter
            .split("|")
            .map(|v| v.to_string())
            .collect();
    }

    pub fn is_target_font_family(&self, family: &String) -> bool {
        if self.font_family_filter_pattern.is_empty() {
            return true;
        }
        for p in &self.font_family_filter_pattern {
            if family.contains(p) {
                return true;
            }
        }
        false
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
                self.compile_font_family_filter();
                return Task::done(Message::UpdateShownFontFamily);
            }
            Message::Pin(id) => {
                if self.pinned.contains(&id) {
                    let _: Vec<usize> = self.pinned.extract_if(.., |i| *i == id).collect();
                } else {
                    self.pinned.push(id);
                    self.pinned.sort();
                    self.pinned.dedup();
                }
                return Task::done(Message::UpdateShownFontFamily);
            }
            Message::HideUnpin(v) => {
                self.hide_unpin = v;
                return Task::done(Message::UpdateShownFontFamily);
            }
            Message::UpdateShownFontFamily => {
                self.shown_family = self
                    .font_families
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| self.is_target_font_family(v))
                    .map(|(i, _)| i)
                    .collect();
                if self.hide_unpin {
                    let filtered_families =
                        HashSet::<_>::from_iter(self.shown_family.iter().map(|v| *v as u64));
                    let pinned = HashSet::from_iter(self.pinned.iter().map(|v| *v as u64));
                    self.shown_family = filtered_families
                        .intersection(&pinned)
                        .map(|v| usize::try_from(*v).unwrap())
                        .collect();
                }
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
                .width(Length::Fixed(120f32))
            ],
            widget::row![
                "Size:",
                widget::text_input("text size", self.text_size.to_string().as_str())
                    .on_input(|v| Message::TextSizeChanged(v.parse().unwrap_or(self.text_size)))
                    .width(Length::Fixed(60f32)),
                widget::slider(10f32..=200f32, self.text_size, Message::TextSizeChanged)
            ],
            widget::row![
                "Font Family Filter:",
                widget::text_input("font family", &self.font_family_filter)
                    .on_input(Message::FontFamilyFilterChanged)
            ],
            widget::row![
                "pinned",
                widget::toggler(self.hide_unpin).on_toggle(Message::HideUnpin)
            ],
            scrollable(iced::widget::table(
                vec![
                    widget::table::column(th("Font Family"), |i: &usize| {
                        widget::text(self.font_families.get(*i).unwrap())
                    }),
                    widget::table::column(th("Text"), |i: &usize| {
                        widget::text(self.text.as_str())
                            .size(self.text_size)
                            .font(Font {
                                weight: self.weight.clone().into(),
                                ..Font::with_name(self.font_families.get(*i).unwrap())
                            })
                    })
                    .width(Length::Fill),
                    widget::table::column(th("Pin"), |i: &usize| {
                        widget::toggler(self.pinned.contains(i)).on_toggle(|_| Message::Pin(*i))
                    })
                ],
                self.shown_family.iter(),
            ))
            .spacing(3)
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
