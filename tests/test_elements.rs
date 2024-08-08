#[cfg(test)]
mod test_elements {
    use visualife::shapes::{Circle, Path, Rect};
    use visualife::ToSvg;

    #[test]
    fn test_circle() {
        let c = Circle::new("c1", 100.0, 100.0, 10.0);
        assert_eq!(c.to_svg(), r#"<circle id="c1" cx="100" cy="100" r="10" />"#);
    }

    #[test]
    fn test_rectangle() {
        let r = Rect::new("r1", 100.0, 100.0, 10.0, 10.0);
        assert_eq!(r.to_svg(), r#"<rect id="r1" x="100" y="100" width="10" height="10" />"#);
    }

    #[test]
    fn test_path() {
        let mut p = Path::new("p1", "M 100 100 L 300 100 L 200 300 z".to_string());
        assert_eq!(p.to_svg(), r#"<path id="p1" d="M 100 100 L 300 100 L 200 300 z" />"#);

        p.style.set_stroke("#000000");
        assert_eq!(p.to_svg(), r#"<path id="p1" d="M 100 100 L 300 100 L 200 300 z" style="stroke:#000000;" />"#);
    }
}