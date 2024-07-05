pub fn bisect<F>(f: F, bound: (f64, f64), max_iter: usize) -> Result<f64, String>
where
    F: Fn(f64) -> f64,
{
    let eps = 1e-10;
    let (mut a, mut b) = bound;
    if a >= b {
        return Err(format!("a < b, a = {a}, b = {b}"));
    }
    let (mut fa, mut fb) = (f(a), f(b));
    if (fa > 0.0) == (fb > 0.0) {
        return Err(format!("sign(f(a)) != sign(f(b)), a = {a}, b = {b}"));
    }

    for i in 0..max_iter {
        if cfg!(test) {
            println!("{i}: [a, b] = [{a}, {b}]");
        }

        let m = (a + b) / 2.0;
        let fm = f(m);
        if f64::abs(a - b) / 2.0 < eps || fm.abs() < eps {
            return Ok(m);
        }
        (a, b, fa, fb) = match (fm > 0.0) == (fb > 0.0) {
            true => (a, m, fa, fm),
            false => (m, b, fm, fb),
        };
    }

    Err(format!("max_iter={max_iter}, [a, b] = [{a}, {b}]"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bisection_1() {
        let r = bisect(
            |x: f64| x.powf(5.0) + x.powf(4.0) - 4.0 * x.powf(3.0) + 3.0 * x.powf(2.0) - 5.0,
            (-2.0, 0.0),
            100,
        )
        .unwrap();
        assert_eq!(r, -0.8714771153754555);
    }

    #[test]
    fn bisection_2() {
        let r = bisect(
            |x: f64| x.powf(5.0) + x.powf(4.0) - 4.0 * x.powf(3.0) + 3.0 * x.powf(2.0) - 5.0,
            (-2.0, 0.0),
            30,
        );
        assert!(r.is_err())
    }

    #[test]
    fn bisection_3() {
        let r = bisect(
            |x: f64| x.powf(5.0) + x.powf(4.0) - 4.0 * x.powf(3.0) + 3.0 * x.powf(2.0) - 5.0,
            (2.0, 0.0),
            100,
        );
        assert!(r.is_err())
    }

    #[test]
    fn bisection_4() {
        let r = bisect(
            |x: f64| x.powf(5.0) + x.powf(4.0) - 4.0 * x.powf(3.0) + 3.0 * x.powf(2.0) - 5.0,
            (2.0, -1.0),
            100,
        );
        assert!(r.is_err())
    }
}
