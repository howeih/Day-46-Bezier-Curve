extern crate gnuplot;
use gnuplot::Figure;

fn binom(n: u32, k: u32) -> u32 {
    if k == 0 {
        return 1;
    }
    if n == 0 && k != 0 {
        return 0;
    }
    binom(n - 1, k - 1) + binom(n - 1, k)
}

fn bezier(points: &[(i32, i32)], steps: usize) -> Vec<(f64, f64)> {
    let n = points.len();
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = binom((n - 1) as u32, i as u32);
    }
    let r: Vec<usize> = (0..n).collect();
    let mut t = 0.;
    let mut res = Vec::<(f64, f64)>::new();
    for _ in 0..steps {
        let u: Vec<f64> = r.iter().map(|r| (t as f64).powf(*r as f64)).collect();
        let n_r: Vec<f64> = r
            .iter()
            .map(|r| (1. - t as f64).powf(n as f64 - *r as f64 - 1.))
            .collect();
        let dot = u
            .iter()
            .zip(n_r.iter())
            .map(|x| x.0 * x.1)
            .zip(b.iter())
            .map(|z| z.0 * (*z.1 as f64))
            .zip(points.iter())
            .map(|p| (p.0 * (p.1).0 as f64, p.0 * (p.1).1 as f64))
            .fold((0., 0.), |x, y| (x.0 + y.0, x.1 + y.1));
        t += 1. / (steps as f64 - 1.);
        res.push(dot);
    }
    res
}

fn main() {
    let point = vec![(0, 0), (0, 1), (1, 1), (1, 0)];
    let mut points = Vec::<(i32, i32)>::with_capacity(16);
    for _ in 0..4 {
        points.append(&mut point.clone());
    }
    let mut fg = Figure::new();
    {
        let axes2d = fg.axes2d();
        for i in 3..14 {
            let curve = bezier(&points[0..i], 100);
            let x: Vec<f64> = curve.iter().map(|x| x.0).collect();
            let y: Vec<f64> = curve.iter().map(|y| y.1).collect();
            axes2d.points(x, y, &[]);
        }
    }
    fg.show();
}
