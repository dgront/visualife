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
        let mut p = Path::from_str("p1", "M 100 100 L 300 100 L 200 300 Z");
        assert_eq!(p.to_svg(), r#"<path id="p1" d="M 100 100 L 300 100 L 200 300 Z " />"#);

        p.style.set_stroke("#000000");
        assert_eq!(p.to_svg(), r#"<path id="p1" d="M 100 100 L 300 100 L 200 300 Z " style="stroke:#000000;" />"#);

        let p = Path::new("p1").move_to(100.0, 100.0).line_to(300.0, 100.0).line_to(200.0, 300.0).close();
        assert_eq!(p.to_svg(), r#"<path id="p1" d="M 100 100 L 300 100 L 200 300 Z " />"#);
    }

    #[test]
    fn test_group() {
        use visualife::shapes::{Circle, Group};
        use visualife::style::Style;
        use visualife::ToSvg;
        let mut g = Group::new("my_group");
        g.add_element(Box::new(Circle::new("my_circle", 100.0, 50.0, 10.0)));
        g.add_element(Box::new(Circle::new("my_circle", 100.0, 100.0, 10.0)));
        let svg = g.to_svg();
        let expected1 = r#"<g id="my_group">
	<circle id="my_circle" cx="100" cy="50" r="10" />
	<circle id="my_circle" cx="100" cy="100" r="10" />
</g>"#;
        assert_eq!(svg, expected1);
        println!("{}", svg);
    }
}