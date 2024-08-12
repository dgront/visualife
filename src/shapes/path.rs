use crate::draw_svg::{close_element, ToSvg};
use crate::style::Style;

#[derive(Debug, Clone)]
pub struct Path {
    pub id: String,
    pub d: String,
    pub style: Style,
}

impl Path {
    pub fn new(id: &str, d: &str) -> Self {
        Path { id: id.to_string(), d: d.to_string(), style: Style::new(), }
    }
}

impl ToSvg for Path {
    fn to_svg(&self) -> String {
        let mut svg_string = format!(r#"<path id="{}" d="{}""#, self.id, self.d);
        close_element(&self.style, &mut svg_string);
        svg_string
    }
}