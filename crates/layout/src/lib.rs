/// Juniper layout stubs: logical layout -> physical boxes
use juniper_dom::Rdom;

#[derive(Debug, Clone)]
pub struct LogicalBlock {
    pub id: usize,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct PhysicalBox {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct PhysicalDoc {
    pub boxes: Vec<PhysicalBox>,
}

pub fn layout(rdom: &Rdom) -> PhysicalDoc {
    let boxes = rdom
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| PhysicalBox {
            x: 0.0,
            y: (i as f32) * 10.0,
            w: 100.0,
            h: 10.0,
            content: n.text.clone(),
        })
        .collect();
    PhysicalDoc { boxes }
}

#[cfg(test)]
mod tests {
    use super::*;
    use juniper_dom::build_rdom;
    use juniper_parser::parse_source;

    #[test]
    fn layout_simple() {
        let ast = parse_source("x y").unwrap();
        let rdom = build_rdom(&ast);
        let pd = layout(&rdom);
        assert_eq!(pd.boxes.len(), 2);
        assert_eq!(pd.boxes[1].content, "y");
    }
}
