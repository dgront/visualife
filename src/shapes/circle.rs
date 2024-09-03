use crate::draw_svg::{close_element, ToSvg};
use crate::style::Style;

/// A ``<circle>`` element
#[derive(Debug, Clone)]
pub struct Circle {
    pub id: String,
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
    pub style: Style,
}

impl Circle {
    pub fn new(id: &str, cx: f32, cy: f32, radius: f32) -> Self {
        Circle { id: id.to_string(), cx, cy, radius, style: Style::new(), }
    }
}


impl ToSvg for Circle {
    fn to_svg(&self) -> String {
        let mut svg_string = format!(
            r#"<circle id="{}" cx="{}" cy="{}" r="{}""#,
            self.id, self.cx, self.cy, self.radius
        );
        close_element(&self.style, &mut svg_string);

        svg_string
    }
}