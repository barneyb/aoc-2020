use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;
use std::hash::Hash;

pub trait AsciiChart {
    fn to_chart_tuples(&self) -> Vec<(String, &usize)>;

    fn render_chart(&self) -> String {
        self.render_chart_within(72)
    }

    #[allow(unused_must_use)]
    fn render_chart_within(&self, max_width: usize) -> String {
        let hist = self.to_chart_tuples();
        let mut s = String::new();
        let (gutter_width, max) = hist
            .iter()
            .fold((0, 0), |(gw, m), t| (gw.max(t.0.len()), m.max(*t.1)));
        let val_len = max.to_string().len();
        let bar_width = (max * 4).min(max_width - gutter_width - 2 - 2 - val_len);
        for t in hist {
            writeln!(
                s,
                "{:>gw$} |{:bw$}| {:>vw$}",
                t.0,
                "#".repeat(t.1 * bar_width / max),
                t.1,
                gw = gutter_width,
                bw = bar_width,
                vw = val_len,
            );
        }
        s
    }
}

impl AsciiChart for Vec<usize> {
    fn to_chart_tuples(&self) -> Vec<(String, &usize)> {
        self.iter()
            .enumerate()
            .map(|(i, c)| (i.to_string(), c))
            .collect::<Vec<(String, &usize)>>()
    }
}

impl<T> AsciiChart for HashMap<T, usize>
where
    T: Eq + Hash + Ord + Display,
{
    fn to_chart_tuples(&self) -> Vec<(String, &usize)> {
        let mut pairs = self.iter().collect::<Vec<(&T, &usize)>>();
        pairs.sort_by_key(|p| p.0);
        pairs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_vec() {
        let hist = vec![17, 40, 10, 12, 13, 14, 0, 98, 74, 12, 0];
        let result = hist.render_chart();
        println!("{}", result);

        let hist = vec![1, 2, 3];
        let result = hist.render_chart_within(50); // won't go that wide
        println!("{}", result);
        assert_eq!(
            result,
            "0 |####        | 1
1 |########    | 2
2 |############| 3
"
        );
    }

    #[test]
    fn test_render_hash_map() {
        let hist: HashMap<&str, usize> = vec![
            ("Barney", 1usize),
            ("Sally", 1),
            ("Jackie", 1),
            ("Johann", 3),
        ]
        .into_iter()
        .collect();
        let result = hist.render_chart();
        println!("{}", result);
        assert_eq!(
            "Barney |####        | 1
Jackie |####        | 1
Johann |############| 3
 Sally |####        | 1
",
            result
        );
    }
}
