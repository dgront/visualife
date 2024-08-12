use crate::style::Style;
use crate::ToSvg;

/// A group of SVG elements.
///
/// # Examples
/// ```
/// use visualife::shapes::{Circle, Group};
/// use visualife::style::Style;
/// use visualife::ToSvg;
/// let mut g = Group::new("my_group");
/// g.add_element(Box::new(Circle::new("my_circle", 100.0, 50.0, 10.0)));
/// g.add_element(Box::new(Circle::new("my_circle", 100.0, 100.0, 10.0)));
/// let svg = g.to_svg();
/// println!("{}", svg);
/// ```
pub struct Group {
    pub id: String,
    pub children: Vec<Box<dyn ToSvg>>,
    pub style: Style,
}

impl Group {
    pub fn new(id: &str) -> Self {
        Group { id: id.to_string(), children: vec![], style: Style::new() }
    }

    pub fn add_element(&mut self, child: Box<dyn ToSvg>) { self.children.push(child); }
}

impl ToSvg for Group {
    fn to_svg(&self) -> String {
        let mut svg_string = format!(r#"<g id="{}""#, self.id);
        if !self.style.is_empty() {
            svg_string.push_str(&format!(r#" style="{}">\n"#, self.style.to_string()));
        } else {
            svg_string.push_str(">\n");
        }
        for child in &self.children {
            svg_string.push_str(&"\t");
            svg_string.push_str(&child.to_svg());
            svg_string.push_str(&"\n");
        }
        svg_string.push_str("</g>");
        return svg_string;
    }
}