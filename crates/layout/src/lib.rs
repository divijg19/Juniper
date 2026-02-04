/// Juniper layout stubs: logical layout -> physical boxes
use juniper_dom::Rdom;
use rustybuzz::{Face, UnicodeBuffer};
use std::env;
use std::fs;

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
    let target = 30usize; // target line width in character cells

    // Process each RDOM node (paragraph) individually using the line breaker.
    let mut boxes = Vec::new();
    for (pidx, node) in rdom.nodes.iter().enumerate() {
        let words: Vec<String> = node
            .text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        let lines = knuth_plass_line_break(&words, target);
        for (i, line) in lines.iter().enumerate() {
            let content = line.join(" ");
            boxes.push(PhysicalBox {
                x: 0.0,
                y: ((pidx * 100) as f32) + (i as f32) * 12.0,
                w: target as f32,
                h: 12.0,
                content,
            });
        }
    }
    PhysicalDoc { boxes }
}

// Public wrapper that will eventually host a full Knuth–Plass implementation.
// For now it delegates to the existing dynamic programming line breaker to
// provide correct results while the full algorithm is implemented incrementally.
fn knuth_plass_line_break(words: &[String], target: usize) -> Vec<Vec<String>> {
    // Full (simplified) Knuth–Plass total-fit implementation with shaping.
    if words.is_empty() {
        return Vec::new();
    }

    // Compute word widths either via rustybuzz shaping if `JUNIPER_FONT_PATH`
    // is set and points to a valid font, otherwise fall back to character
    // counts as an approximate width.
    let mut lens: Vec<f64> = vec![0.0f64; words.len()];
    if let Ok(path) = env::var("JUNIPER_FONT_PATH") {
        if let Ok(data) = fs::read(&path) {
            if let Some(face) = Face::from_slice(&data, 0) {
                let upem = face.units_per_em() as f64;
                let px_per_em = 10.0f64; // nominal design size for metrics
                for (i, w) in words.iter().enumerate() {
                    let mut buf = UnicodeBuffer::new();
                    buf.push_str(w);
                    let shaped = rustybuzz::shape(&face, &[], buf);
                    let mut adv: i32 = 0;
                    for pos in shaped.glyph_positions() {
                        adv += pos.x_advance;
                    }
                    lens[i] = (adv as f64) * (px_per_em / upem);
                }
            } else {
                for (i, w) in words.iter().enumerate() {
                    lens[i] = w.len() as f64;
                }
            }
        } else {
            for (i, w) in words.iter().enumerate() {
                lens[i] = w.len() as f64;
            }
        }
    } else {
        for (i, w) in words.iter().enumerate() {
            lens[i] = w.len() as f64;
        }
    }

    // Precompute prefix sums for quick range widths
    let n = words.len();
    let mut prefix = vec![0.0f64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + lens[i];
    }

    // Glue model (nominal space, stretch, shrink)
    let space_nom = 1.0f64;
    let space_stretch = 3.0f64;
    let space_shrink = 3.0f64;
    let target_f = target as f64;

    #[derive(Clone)]
    struct Breakpoint {
        demerits: f64,
        fitness: usize,
        prev: usize,
    }

    let mut best: Vec<[Option<Breakpoint>; 4]> = vec![[None, None, None, None]; n + 1];
    best[0][1] = Some(Breakpoint {
        demerits: 0.0,
        fitness: 1,
        prev: 0,
    });

    for j in 1..=n {
        for i in 0..j {
            let words_len = prefix[j] - prefix[i];
            let spaces = (j - i).saturating_sub(1) as f64;
            let nominal = words_len + spaces * space_nom;
            let total_stretch = spaces * space_stretch;
            let total_shrink = spaces * space_shrink;

            for fin in 0..4usize {
                if let Some(bp_prev) = &best[i][fin] {
                    let prev_demerits = bp_prev.demerits;

                    let r = if nominal <= target_f {
                        if total_stretch.abs() < 1e-9 {
                            if (target_f - nominal).abs() > 1e-6 {
                                f64::INFINITY
                            } else {
                                0.0
                            }
                        } else {
                            (target_f - nominal) / total_stretch
                        }
                    } else {
                        if total_shrink.abs() < 1e-9 {
                            if (target_f - nominal).abs() > 1e-6 {
                                f64::INFINITY
                            } else {
                                0.0
                            }
                        } else {
                            (target_f - nominal) / total_shrink
                        }
                    };

                    if !r.is_finite() || r < -1.0 {
                        continue;
                    }

                    let badness = {
                        let x = (100.0 * r).abs();
                        let b = x.powi(3);
                        if b > 1e12 { 1e12 } else { b }
                    };

                    let fitness = if r < -0.5 {
                        0usize
                    } else if r <= 0.5 {
                        1usize
                    } else if r <= 1.0 {
                        2usize
                    } else {
                        3usize
                    };

                    let penalty = 0.0f64;
                    let mut demerits = prev_demerits + (badness + penalty).powi(2);
                    if (bp_prev.fitness as i32 - fitness as i32).abs() > 1 {
                        demerits += 10000.0;
                    }

                    match &best[j][fitness] {
                        Some(existing) => {
                            if demerits < existing.demerits {
                                best[j][fitness] = Some(Breakpoint {
                                    demerits,
                                    fitness,
                                    prev: i,
                                });
                            }
                        }
                        None => {
                            best[j][fitness] = Some(Breakpoint {
                                demerits,
                                fitness,
                                prev: i,
                            });
                        }
                    }
                }
            }
        }
    }

    // pick best final
    let mut best_final: Option<(f64, usize)> = None;
    for (f, opt_bp) in best[n].iter().enumerate().take(4) {
        if let Some(bp) = opt_bp
            && (best_final.is_none() || bp.demerits < best_final.unwrap().0)
        {
            best_final = Some((bp.demerits, f));
        }
    }
    if best_final.is_none() {
        return line_break(words, target);
    }

    let mut lines = Vec::new();
    let mut j = n;
    let mut fitness = best_final.unwrap().1;
    while j > 0 {
        let bp = best[j][fitness].as_ref().unwrap();
        let prev = bp.prev;
        lines.push(words[prev..j].to_vec());

        // pick previous fitness with minimal demerits
        let mut next_f = 0usize;
        let mut min_dem = f64::INFINITY;
        for (f, opt_bp_prev) in best[prev].iter().enumerate().take(4) {
            if let Some(bp_prev) = opt_bp_prev
                && bp_prev.demerits < min_dem
            {
                min_dem = bp_prev.demerits;
                next_f = f;
            }
        }
        fitness = next_f;
        j = prev;
    }
    lines.reverse();
    lines
}

// A simplified Knuth–Plass-like line breaking implementation.
// Returns a Vec of lines where each line is Vec<String> of words.
// This implementation uses dynamic programming minimizing cubic badness,
// but treats the last line with a softer penalty to avoid over-penalizing
// natural ragged ends (common in typesetting).
fn line_break(words: &[String], target: usize) -> Vec<Vec<String>> {
    let n = words.len();
    if n == 0 {
        return Vec::new();
    }
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
            // Full (simplified) Knuth–Plass total-fit implementation.
            let remaining = target as isize - width as isize;
            // badness: cubic for non-final lines, quadratic for final line
            let badness = if j == words.len() {
                // prefer softer penalty for final line
                (remaining as f64).powi(2)
            } else {
                (remaining as f64).powi(3)
            };
            let candidate = cost[i] + badness;
            if candidate < cost[j] {
                cost[j] = candidate;
                prev[j] = i;
            }
        }
        // allow overflow lines (forced) if no candidate found
        if cost[j].is_infinite() {
            // place last word on its own line (overflow)
            cost[j] = cost[j - 1] + 1e9;
            prev[j] = j - 1;
        }
    }

    // reconstruct lines
    let mut lines: Vec<Vec<String>> = Vec::new();
    let mut idx = n;
    while idx > 0 {
        let i = prev[idx];
        let mut line = Vec::new();
        for w in &words[i..idx] {
            line.push(w.clone());
        }
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
        // with target small, both words may fit on a single line
        assert!(pd.boxes.len() >= 1);
        assert!(pd.boxes[0].content.contains("x"));
    }

    #[test]
    fn layout_multiline() {
        // create a long input dynamically to guarantee multiple lines
        let long = (0..60)
            .map(|i| format!("word{}", i))
            .collect::<Vec<_>>()
            .join(" ");
        let ast = parse_source(&long).unwrap();
        let rdom = build_rdom(&ast);
        let pd = layout(&rdom);
        // expect more than one line for this long input
        assert!(
            pd.boxes.len() >= 2,
            "expected multiple lines, got {}",
            pd.boxes.len()
        );
    }

    #[test]
    fn long_word_overflow() {
        // a single word longer than target should still produce a box (overflow)
        let long_word = std::iter::repeat('a').take(120).collect::<String>();
        let ast = parse_source(&long_word).unwrap();
        let rdom = build_rdom(&ast);
        let pd = layout(&rdom);
        assert_eq!(pd.boxes.len(), 1);
        assert!(pd.boxes[0].content.contains(&long_word));
    }

    #[test]
    fn many_short_words_generate_multiple_lines() {
        let long = (0..200)
            .map(|_| "x".to_string())
            .collect::<Vec<_>>()
            .join(" ");
        let ast = parse_source(&long).unwrap();
        let rdom = build_rdom(&ast);
        let pd = layout(&rdom);
        assert!(
            pd.boxes.len() >= 3,
            "expected multiple lines, got {}",
            pd.boxes.len()
        );
    }
}
