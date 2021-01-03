use crate::ascii::chart::PlotStyle::{Bar, Point};
use num_traits::{Num, NumOps, ToPrimitive};
use std::collections::HashMap;
use std::fmt::Display;
use std::fmt::Write;
use std::hash::Hash;

enum PlotStyle {
    Bar,
    Point,
}

struct Shape {
    width: usize,
    max_label_len: usize,
    max_val_len: usize,
    min_val: f64,
    max_val: f64,
    plot_style: PlotStyle,
}

impl Shape {
    fn point(width: usize) -> Shape {
        Shape {
            width,
            max_label_len: 0,
            max_val_len: 0,
            min_val: f64::MAX,
            max_val: f64::MIN,
            plot_style: Point,
        }
    }

    fn bar(width: usize) -> Shape {
        Shape {
            min_val: 0f64,
            max_val: 0f64,
            plot_style: Bar,
            ..Shape::point(width)
        }
    }

    fn range(&self) -> f64 {
        self.max_val - self.min_val
    }
}

#[allow(unused_must_use)]
fn do_render(shape: Shape, tuples: Vec<(&String, f64)>) -> String {
    let shape = tuples.iter().fold(shape, |s, &(lbl, n)| Shape {
        max_label_len: s.max_label_len.max(lbl.len()),
        max_val_len: s.max_val_len.max(n.to_string().len()),
        min_val: s.min_val.min(n),
        max_val: s.max_val.max(n),
        ..s
    });
    let bar_width = shape.width - shape.max_label_len - 2 - 2 - shape.max_val_len;
    let mut s = String::new();
    for (lbl, n) in tuples {
        let plot = match shape.plot_style {
            Bar => "#"
                .repeat((((n - shape.min_val) / shape.range()) * bar_width as f64).ceil() as usize),
            Point => {
                " ".repeat(
                    (((n - shape.min_val) / shape.range()) * (bar_width - 1) as f64) as usize,
                ) + "*"
            }
        };
        writeln!(
            s,
            "{:>gw$} |{:bw$}| {:>vw$}",
            lbl,
            plot,
            n,
            gw = shape.max_label_len,
            bw = bar_width,
            vw = shape.max_val_len,
        );
    }
    let min = shape.min_val.to_string();
    writeln!(
        s,
        "{:gw$} |{}{:>bw$}|",
        "",
        min,
        shape.max_val,
        gw = shape.max_label_len,
        bw = bar_width - min.len(),
    );
    s
}

fn tuples_to_f64<N>(tuples: &Vec<(String, N)>) -> Vec<(&String, f64)>
where
    N: ToPrimitive,
{
    tuples
        .iter()
        .map(|(lbl, n)| (lbl, n.to_f64().unwrap()))
        .collect()
}

pub trait AsciiChart<N>
where
    N: Num + NumOps + ToPrimitive,
{
    fn to_chart_tuples(&self) -> Vec<(String, N)>;

    fn render_plot(&self) -> String {
        let raw_tuples = self.to_chart_tuples();
        let chart_tuples = tuples_to_f64(&raw_tuples);
        do_render(Shape::point(72), chart_tuples)
    }

    fn render_histogram(&self) -> String {
        let raw_tuples = self.to_chart_tuples();
        let chart_tuples = tuples_to_f64(&raw_tuples);
        do_render(Shape::bar(72), chart_tuples)
    }
}

impl<N> AsciiChart<N> for Vec<N>
where
    N: Copy + Num + NumOps + ToPrimitive,
{
    fn to_chart_tuples(&self) -> Vec<(String, N)> {
        self.iter()
            .enumerate()
            .map(|(i, &c)| (i.to_string(), c))
            .collect::<Vec<(String, N)>>()
    }
}

impl<T, N> AsciiChart<N> for HashMap<T, N>
where
    T: Eq + Hash + Ord + Display,
    N: Copy + Num + NumOps + ToPrimitive,
{
    fn to_chart_tuples(&self) -> Vec<(String, N)> {
        let mut pairs = self.iter().collect::<Vec<(&T, &N)>>();
        pairs.sort_by_key(|p| p.0);
        pairs.iter().map(|&(k, &v)| (k.to_string(), v)).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_vec() {
        let hist: Vec<usize> = vec![17, 40, 10, 12, 13, 14, 0, 98, 74, 12, 0];
        let result = hist.render_histogram();
        println!("{}", result);

        let hist = vec![1, 2, 3];
        let result = hist.render_histogram();
        println!("{}", result);
        assert_eq!(
            result,
            "0 |######################                                            | 1
1 |############################################                      | 2
2 |##################################################################| 3
  |0                                                                3|
"
        );
    }

    #[test]
    fn i32_vec() {
        let the_data: Vec<i32> = vec![-3, 7, 2];
        println!("{:?}", the_data);
        println!("{}", the_data.render_plot());
    }

    #[test]
    fn u32_vec() {
        let the_data: Vec<u32> = vec![7, 4, 12];
        println!("{:?}", the_data);
        println!("{}", the_data.render_plot());
        println!("{}", the_data.render_histogram());
    }

    #[test]
    fn f64_vec() {
        let mut the_data = vec![-3.5, -2f64, -1.3];
        println!("{:?}", the_data);
        println!("{}", the_data.render_plot());
        the_data.push(0.123);
        println!("{:?}", the_data);
        println!("{}", the_data.render_plot());
    }

    #[test]
    fn i32_map() {
        let timber_resources: HashMap<&str, i32> =
            [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
                .iter()
                .cloned()
                .collect();
        println!("{:?}", timber_resources);
        println!("{}", timber_resources.render_histogram());
    }

    #[test]
    fn f32_map() {
        let avg_temp: HashMap<&str, _> = [("Norway", -10.5), ("Denmark", 8.4), ("Iceland", -30.0)]
            .iter()
            .cloned()
            .collect();
        println!("{:?}", avg_temp);
        let result = avg_temp.render_plot();
        println!("{}", result);
        assert_eq!(
            "Denmark |                                                       *|   8.4
Iceland |*                                                       |   -30
 Norway |                           *                            | -10.5
        |-30                                                  8.4|
",
            result
        )
    }

    #[test]
    fn test_render_hash_map() {
        let hist: HashMap<&str, usize> =
            vec![("Barney", 1), ("Sally", 1), ("Jackie", 1), ("Johann", 3)]
                .into_iter()
                .collect();
        let result = hist.render_histogram();
        println!("{}", result);
        assert_eq!(
            "Barney |#####################                                        | 1
Jackie |#####################                                        | 1
Johann |#############################################################| 3
 Sally |#####################                                        | 1
       |0                                                           3|
",
            result
        );
    }
}
