use std::fmt::{Display, Formatter};
use crate::draw_svg::{close_element, ToSvg};
use crate::style::Style;

#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    HorizontalTo(f32),
    VerticalTo(f32),
    CurveTo(f32, f32, f32, f32, f32, f32),
    SmoothCurveTo(f32, f32, f32, f32),
    QuadraticBezierCurveTo(f32, f32, f32, f32),
    SmoothQuadraticBezierCurveTo(f32, f32),
    EllipticalArcTo(f32, f32, f32, bool, bool, f32, f32),
    Close,
}

impl Display for PathCommand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PathCommand::MoveTo(x, y) => write!(f, "M {} {}", x, y),
            PathCommand::LineTo(x, y) => write!(f, "L {} {}", x, y),
            PathCommand::HorizontalTo(x) => write!(f, "H {}", x),
            PathCommand::VerticalTo(y) => write!(f, "V {}", y),
            PathCommand::CurveTo(x1, y1, x2, y2, x, y) => write!(f, "C {} {} {} {} {} {}", x1, y1, x2, y2, x, y),
            PathCommand::SmoothCurveTo(x2, y2, x, y) => write!(f, "S {} {} {} {}", x2, y2, x, y),
            PathCommand::QuadraticBezierCurveTo(x1, y1, x, y) => write!(f, "Q {} {} {} {}", x1, y1, x, y),
            PathCommand::SmoothQuadraticBezierCurveTo(x, y) => write!(f, "T {} {}", x, y),
            PathCommand::EllipticalArcTo(rx, ry, x_axis_rotation, large_arc_flag , sweep_flag, x, y)
                => write!(f, "A {} {} {} {} {} {} {}", rx, ry, x_axis_rotation, (*large_arc_flag as i32), (*sweep_flag as i32), x, y),
            PathCommand::Close => { write!(f, "Z") }
        }
    }
}

/// A ``<path>`` element
///
/// Paths are a series of commands that describe how to draw a shape.
///
/// # Example
/// ```rust
/// use visualife::shapes::{Path};
/// use visualife::ToSvg;
///
/// let mut p = Path::new("a_path")
///     .move_to(50.0, 50.0)
///     .line_to(10.0, 10.0);
/// assert_eq!(p.to_svg(), r#"<path id="a_path" d="M 50 50 L 10 10 " />"#);
/// ```
#[derive(Debug, Clone)]
pub struct Path {
    pub id: String,
    pub d: Vec<PathCommand>,
    pub style: Style,
}

impl Path {
    pub fn new(id: &str) -> Self { Path { id: id.to_string(), d: vec![], style: Style::new(), } }

    pub fn line_to(mut self, x: f32, y: f32) -> Self { self.add_command(PathCommand::LineTo(x, y)) }
    pub fn move_to(mut self, x: f32, y: f32) -> Self { self.add_command(PathCommand::MoveTo(x, y)) }
    pub fn horizontal_to(mut self, x: f32) -> Self { self.add_command(PathCommand::HorizontalTo(x)) }
    pub fn vertical_to(mut self, y: f32) -> Self { self.add_command(PathCommand::VerticalTo(y)) }
    pub fn curve_to(mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) -> Self {
        self.add_command(PathCommand::CurveTo(x1, y1, x2, y2, x, y))
    }
    pub fn smooth_curve_to(mut self, x2: f32, y2: f32, x: f32, y: f32) -> Self {
        self.add_command(PathCommand::SmoothCurveTo(x2, y2, x, y))
    }
    pub fn quadratic_bezier_curve_to(mut self, x1: f32, y1: f32, x: f32, y: f32) -> Self {
        self.add_command(PathCommand::QuadraticBezierCurveTo(x1, y1, x, y))
    }
    pub fn smooth_quadratic_bezier_curve_to(mut self, x: f32, y: f32) -> Self {
        self.add_command(PathCommand::SmoothQuadraticBezierCurveTo(x, y))
    }
    pub fn elliptical_arc_to(mut self, rx: f32, ry: f32, x_axis_rotation: f32, large_arc_flag: bool,
                             sweep_flag: bool, x: f32, y: f32) -> Self {
        self.add_command(PathCommand::EllipticalArcTo(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y))
    }
    pub fn close(mut self) -> Self { self.add_command(PathCommand::Close) }

    pub fn add_command(mut self, command: PathCommand) -> Self {
        self.d.push(command);
        return self;
    }
}

impl ToSvg for Path {
    fn to_svg(&self) -> String {
        let mut d_str = String::new();
        for command in &self.d {
            d_str.push_str(&format!("{} ", command));
        }
        let mut svg_string = format!(r#"<path id="{}" d="{}""#, self.id, d_str);
        close_element(&self.style, &mut svg_string);
        svg_string
    }
}