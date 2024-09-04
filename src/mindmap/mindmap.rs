use std::collections::HashMap;
use crate::mindmap::connector::connector;
use crate::mindmap::node::Node;
use crate::mindmap::polar_to_cartesian;
use crate::shapes::{Group};
use crate::ToSvg;

#[derive(Debug, Clone)]
pub struct Mindmap {
    pub foot_angle_deg: f32,
    pub bar_width: f32,
    pub id: String,
    nodes: HashMap<String, Node>,
    connections: Vec<(String,String)>,
    max_node_radius: f32,
    node_radius_shrink_factor: f32,
}

impl Mindmap {
    pub fn new(id: &str, max_node_radius: f32) -> Self {
        Mindmap {
            foot_angle_deg: 30.0,
            bar_width: max_node_radius / 5.0,
            id: id.to_string(),
            nodes: HashMap::new(),
            max_node_radius,
            connections: vec![],
            node_radius_shrink_factor: 0.8 }
    }

    pub fn place_node(&mut self, id: &str, label: &str, x: f32, y: f32) {
        let el = Node::new(id, label, x, y, self.max_node_radius);
        self.nodes.insert(id.to_string(), el);
    }

    pub fn grow_node(&mut self, id: &str, label: &str, angle_deg: f32, parent_node_id: &str) {
        let parent = self.nodes.get(parent_node_id).unwrap();
        let (cx, cy) = polar_to_cartesian(parent.radius*3.0, angle_deg, parent.cx, parent.cy);
        let el = Node::new(id, label, cx, cy, parent.radius * self.node_radius_shrink_factor);
        self.nodes.insert(id.to_string(), el);

        self.connect_nodes(parent_node_id, id);
    }

    pub fn connect_nodes(&mut self, from_id: &str, to_id: &str) {
        self.connections.push((from_id.to_string(), to_id.to_string()));
    }
}


impl ToSvg for Mindmap {
    fn to_svg(&self) -> String {
        let mut mindmap_group = Group::new(&format!("{}", self.id));
        let mut node_grp = Group::new(&format!("nodes-{}", self.id));
        for node in self.nodes.values() {
            node_grp.add_element(Box::new(node.clone()));
        }
        let mut connector_grp = Group::new(&format!("connectors-{}", self.id));
        for (from_id, to_id) in &self.connections {
            let from_node = self.nodes.get(from_id).unwrap();
            let to_node = self.nodes.get(to_id).unwrap();
            connector_grp.add_element(Box::new(connector(from_node, to_node, self.foot_angle_deg, self.bar_width)));
        }
        mindmap_group.add_element(Box::new(node_grp));
        mindmap_group.add_element(Box::new(connector_grp));

        return mindmap_group.to_svg();
    }
}