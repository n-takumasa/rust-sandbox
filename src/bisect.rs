use std::ops::ControlFlow;

struct Context {
    a: f64,
    b: f64,
    fa: f64,
    fb: f64,
}

impl Context {
    fn new(a: f64, b: f64, fa: f64, fb: f64) -> Self {
        Self { a, b, fa, fb }
    }
}

fn _bisect<F>(f: F, ctx: Context, eps: f64) -> ControlFlow<(f64, f64), Context>
where
    F: Fn(f64) -> f64,
{
    let m = (ctx.a + ctx.b) / 2.0;
    let fm = f(m);
    if f64::abs(ctx.a - ctx.b) / 2.0 < eps || fm.abs() < eps {
        return ControlFlow::Break((m, fm));
    }
    ControlFlow::Continue(match (fm > 0.0) == (ctx.fb > 0.0) {
        true => Context::new(ctx.a, m, ctx.fa, fm),
        false => Context::new(m, ctx.b, fm, ctx.fb),
    })
}

pub fn bisect<F>(f: F, bound: (f64, f64), max_iter: usize) -> Result<f64, String>
where
    F: Fn(f64) -> f64,
{
    let eps = 1e-10;
    let (a, b) = bound;
    if a >= b {
        return Err(format!("a < b, a = {a}, b = {b}"));
    }
    let (fa, fb) = (f(a), f(b));
    if (fa > 0.0) == (fb > 0.0) {
        return Err(format!("sign(f(a)) != sign(f(b)), a = {a}, b = {b}"));
    }

    match (0..max_iter).try_fold(Context::new(a, b, fa, fb), |ctx, _| _bisect(&f, ctx, eps)) {
        ControlFlow::Break((m, _fm)) => Ok(m),
        ControlFlow::Continue(ctx) => Err(format!(
            "max_iter={max_iter}, [a, b] = [{}, {}]",
            ctx.a, ctx.b
        )),
    }
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
