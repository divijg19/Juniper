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
    // Convert RDOM node texts into a sequence of words for line breaking.
    let words: Vec<String> = rdom.nodes.iter().map(|n| n.text.clone()).collect();
    let target = 30usize; // target line width in character cells
    let lines = line_break(&words, target);

    // Create simple boxes: one box per output line.
    let mut boxes = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        let content = line.join(" ");
        boxes.push(PhysicalBox {
            x: 0.0,
            y: (i as f32) * 12.0,
            w: target as f32,
            h: 12.0,
            content,
        });
    }
    PhysicalDoc { boxes }
}

// A simplified Knuth–Plass line breaking implementation.
// Returns a Vec of lines where each line is Vec<String> of words.
fn line_break(words: &[String], target: usize) -> Vec<Vec<String>> {
    let n = words.len();
    if n == 0 { return Vec::new(); }
    // precompute word lengths
    let lens: Vec<usize> = words.iter().map(|w| w.len()).collect();

    // cost[i] = minimal cost to break first i words
    let mut cost = vec![f64::INFINITY; n + 1];
    let mut prev = vec![0usize; n + 1];
    cost[0] = 0.0;

    for j in 1..=n {
        let mut width = 0usize;
        for i in (0..j).rev() {
            // words i..j-1 on a line
            if i == j - 1 {
                width = lens[i];
            } else {
                width += 1 + lens[i]; // add space + word
            }
            if width > target {
                // too long, stop increasing i
                break;
            }
            let remaining = target as isize - width as isize;
            let badness = (remaining as f64).powi(3);
            let candidate = cost[i] + badness;
            if candidate < cost[j] {
                cost[j] = candidate;
                prev[j] = i;
            }
        }
        // allow overflow lines (forced) if no candidate found
        if cost[j].is_infinite() {
            // place last word on its own line (overflow)
            cost[j] = cost[j-1] + 1e9;
            prev[j] = j-1;
        }
    }

    // reconstruct lines
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut idx = n;
    while idx > 0 {
        let i = prev[idx];
        let mut line = Vec::new();
        for k in i..idx { line.push(words[k].clone()); }
        lines.push(line);
        idx = i;
    }
    lines.reverse();
    lines
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
