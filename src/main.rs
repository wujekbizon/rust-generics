use num_traits::ToPrimitive;

fn solve<T: ToPrimitive, U: ToPrimitive>(a: T, b: U) -> f64 {
    let a_f64 = a.to_f64().unwrap();
    let b_f64 = b.to_f64().unwrap();

    (a_f64.powi(2) + b_f64.powi(2)).sqrt()
}

fn main() {
    let a: i32 = 3;
    let b: f32 = 4.0;

    println!("{}", solve(a, b));
}
