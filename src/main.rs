fn main() {
    println!("Hello, world!");
    println!("Hello, {}!", "ocha");

    let a: f64 = 1.0;
    let b: f64 = 2f64;
    println!("{:.1} + {:.1} = {:.1}", a, b, add(a, b));

    println!(
        "半径 {:.1}、円周率 {:.3}、面積 {:.3}",
        3.2,
        std::f64::consts::PI,
        3.2f64.powi(2) * std::f64::consts::PI,
    )
}

fn add(x: f64, y: f64) -> f64 {
    x + y
}
