use crate::shapes::Circle;
use crate::ToSvg;

#[derive(Debug, Clone)]
pub(crate) struct Node {
    pub id: String,
    pub label: String,
    pub cx: f32,
    pub cy: f32,
    pub radius: f32,
}

impl Node {
    pub fn new(id: &str, label: &str, cx: f32, cy: f32, radius: f32) -> Self {
        return Node { id: id.to_string(), label: label.to_string(), cx, cy, radius }
    }
}

impl ToSvg for Node {
    fn to_svg(&self) -> String {
        Circle::new(&self.id, self.cx, self.cy, self.radius).to_svg()
    }
}

#[cfg(test)]
mod test_node {
    use crate::mindmap::node::Node;
    use crate::ToSvg;

    #[test]
    fn node_to_svg() {
        let (x1, y1, r1)  = (100.0_f32, 100.0_f32, 10.0_f32);
        let na = Node::new("a", "A", x1, y1, r1);
        assert_eq!(na.to_svg(), r#"<circle id="a" cx="100" cy="100" r="10" />"#);
    }
}