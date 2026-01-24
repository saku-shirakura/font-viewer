use iced_graphics::text::font_system;
use std::sync::OnceLock;

fn font_list() -> Vec<String> {
    let mut font_list: Vec<String> = match font_system().write() {
        Ok(mut v) => v
            .raw()
            .db()
            .faces()
            .filter_map(|v| v.families.first().and_then(|v| Some(v.0.clone())))
            .collect(),
        Err(_) => Default::default(),
    };
    font_list.sort();
    font_list.dedup();
    font_list
}

pub fn get_global_font_list() -> &'static Vec<String> {
    static FONT_LIST: OnceLock<Vec<String>> = OnceLock::new();
    FONT_LIST.get_or_init(|| font_list())
}
