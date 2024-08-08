use crate::draw_svg::ToSvg;
use crate::shapes::Circle;
use crate::style::Style;

#[derive(Debug, Clone)]
pub struct Path {
    pub id: String,
    pub d: String,
    pub style: Style,
}


impl Path {
    pub fn new(id: &str, d: String) -> Self {
        Path { id: id.to_string(), d, style: Style::new(), }
    }
}

impl ToSvg for Path {
    fn to_svg(&self) -> String {
        let mut svg_string = format!(
            r#"<path id="{}" d="{}""#, self.id, self.d);

        let style_string = self.style.to_string();
        if !style_string.is_empty() {
            svg_string.push_str(&format!(r#" style="{}""#, style_string));
        }

        svg_string.push_str(r#" />"#);

        svg_string
    }
}