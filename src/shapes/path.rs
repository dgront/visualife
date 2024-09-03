use std::fmt::{Display, Formatter};
use std::str::FromStr;
use crate::draw_svg::{close_element, ToSvg};
use crate::style::Style;

#[derive(Debug, Clone, PartialEq)]
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
    MoveBy(f32, f32),
    LineBy(f32, f32),
    HorizontalBy(f32),
    VerticalBy(f32),
    CurveBy(f32, f32, f32, f32, f32, f32),
    SmoothCurveBy(f32, f32, f32, f32),
    QuadraticBezierCurveBy(f32, f32, f32, f32),
    EllipticalArcBy(f32, f32, f32, bool, bool, f32, f32),
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
            PathCommand::Close => { write!(f, "Z") },
            PathCommand::MoveBy(x, y) => write!(f, "m {} {}", x, y),
            PathCommand::LineBy(x, y) => write!(f, "l {} {}", x, y),
            PathCommand::HorizontalBy(x) => write!(f, "h {}", x),
            PathCommand::VerticalBy(y) => write!(f, "v {}", y),
            PathCommand::CurveBy(x1, y1, x2, y2, x, y) => write!(f, "c {} {} {} {} {} {}", x1, y1, x2, y2, x, y),
            PathCommand::SmoothCurveBy(x2, y2, x, y) => write!(f, "s {} {} {} {}", x2, y2, x, y),
            PathCommand::QuadraticBezierCurveBy(x1, y1, x, y) => write!(f, "q {} {} {} {}", x1, y1, x, y),
            PathCommand::EllipticalArcBy(rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y)
                => write!(f, "a {} {} {} {} {} {} {}", rx, ry, x_axis_rotation, (*large_arc_flag as i32), (*sweep_flag as i32), x, y),
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

    pub fn from_str(id: &str, path: &str) -> Self {
        let mut p = Path::new(id);
        p.d = parse_path_commands(path);

        return p;
    }

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

fn parse_path_commands(path_data: &str) -> Vec<PathCommand> {
    let mut commands = Vec::new();
    let mut chars = path_data.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            'M' | 'm' => {
                chars.next(); // consume 'M' or 'm'
                let coords = parse_numbers(&mut chars, 2);
                if c == 'M' {
                    commands.push(PathCommand::MoveTo(coords[0], coords[1]));
                } else {
                    commands.push(PathCommand::MoveBy(coords[0], coords[1]));
                }
            }
            'L' | 'l' => {
                chars.next(); // consume 'L' or 'l'
                let coords = parse_numbers(&mut chars, 2);
                if c == 'L' {
                    commands.push(PathCommand::LineTo(coords[0], coords[1]));
                } else {
                    commands.push(PathCommand::LineBy(coords[0], coords[1]));
                }
            }
            'H' | 'h' => {
                chars.next(); // consume 'H' or 'h'
                let x = parse_numbers(&mut chars, 1)[0];
                if c == 'H' {
                    commands.push(PathCommand::HorizontalTo(x));
                } else {
                    commands.push(PathCommand::HorizontalBy(x));
                }
            }
            'V' | 'v' => {
                chars.next(); // consume 'V' or 'v'
                let y = parse_numbers(&mut chars, 1)[0];
                if c == 'V' {
                    commands.push(PathCommand::VerticalTo(y));
                } else {
                    commands.push(PathCommand::VerticalBy(y));
                }
            }
            'C' | 'c' => {
                chars.next(); // consume 'C' or 'c'
                let nums = parse_numbers(&mut chars, 6);
                if c == 'C' {
                    commands.push(PathCommand::CurveTo(nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]));
                } else {
                    commands.push(PathCommand::CurveBy(nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]));
                }
            }
            'S' | 's' => {
                chars.next(); // consume 'S' or 's'
                let nums = parse_numbers(&mut chars, 4);
                if c == 'S' {
                    commands.push(PathCommand::SmoothCurveTo(nums[0], nums[1], nums[2], nums[3]));
                } else {
                    commands.push(PathCommand::SmoothCurveBy(nums[0], nums[1], nums[2], nums[3]));
                }
            }
            'Q' | 'q' => {
                chars.next(); // consume 'Q' or 'q'
                let nums = parse_numbers(&mut chars, 4);
                if c == 'Q' {
                    commands.push(PathCommand::QuadraticBezierCurveTo(nums[0], nums[1], nums[2], nums[3]));
                } else {
                    commands.push(PathCommand::QuadraticBezierCurveBy(nums[0], nums[1], nums[2], nums[3]));
                }
            }
            'T' | 't' => {
                chars.next(); // consume 'T' or 't'
                let nums = parse_numbers(&mut chars, 2);
                commands.push(PathCommand::SmoothQuadraticBezierCurveTo(nums[0], nums[1])); // Assuming no 'by' version
            }
            'A' | 'a' => {
                chars.next(); // consume 'A' or 'a'
                let nums = parse_numbers(&mut chars, 7);
                let large_arc = nums[3] != 0.0;
                let sweep = nums[4] != 0.0;
                if c == 'A' {
                    commands.push(PathCommand::EllipticalArcTo(nums[0], nums[1], nums[2], large_arc, sweep, nums[5], nums[6]));
                } else {
                    commands.push(PathCommand::EllipticalArcBy(nums[0], nums[1], nums[2], large_arc, sweep, nums[5], nums[6]));
                }
            }
            'Z' | 'z' => {
                chars.next(); // consume 'Z' or 'z'
                commands.push(PathCommand::Close);
            }
            _ => {
                chars.next(); // ignore unexpected characters
            }
        }
    }

    commands
}

fn parse_numbers(chars: &mut std::iter::Peekable<std::str::Chars>, count: usize) -> Vec<f32> {
    let mut numbers = Vec::new();
    let mut current_number = String::new();

    while numbers.len() < count {
        while let Some(&c) = chars.peek() {
            if c.is_digit(10) || c == '.' || c == '-' {
                current_number.push(c);
                chars.next();
            } else if !current_number.is_empty() {
                break;
            } else {
                chars.next();
            }
        }

        if !current_number.is_empty() {
            if let Ok(num) = f32::from_str(&current_number) {
                numbers.push(num);
            }
            current_number.clear();
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_path_commands() {
        let path_data = "M 100 100 L 300 100 L 200 300 z";
        let expected = vec![
            PathCommand::MoveTo(100.0, 100.0),
            PathCommand::LineTo(300.0, 100.0),
            PathCommand::LineTo(200.0, 300.0),
            PathCommand::Close,
        ];

        let result = parse_path_commands(path_data);
        assert_eq!(result, expected);
    }
}