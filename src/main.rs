use arael_sym::*;
#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
}
fn interpol(points: Vec<Point>) -> E {
    let mut exprs: Vec<E> = vec![];
    for i in 0..points.len() {
        let mut x = symbol("x");
        let mut acc: E = sym! {1.0.into()};
        for (j, pt) in points.iter().enumerate() {
            if i == j {
                continue;
            }
            let term = sym! {(x.clone()-pt.x)/(points[i].x-pt.x)};
            acc = acc * term;
        }
        exprs.push(points[i].y * acc);
    }
    exprs
        .into_iter()
        .reduce(|acc, expr| acc + expr)
        .unwrap()
        .expand()
        .simplify()
}
fn main() {
    let points: Vec<Point> = vec![
        Point { x: 0.0, y: -1.0 },
        Point { x: 1.0, y: 2.0 },
        Point { x: 2.0, y: 7.0 },
        Point { x: 3.0, y: 20.0 },
    ];
    println!("{:?}", interpol(points));
}
