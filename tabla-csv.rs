fn main() {
    let mut f_i: f64;
    for i in (-20..=20).step_by(2) {
        f_i = i as f64;
        println!(
            "{:.3},{:.3},{:.3}",
            (f_i / 10.0),
            (f_i / 10.0) * (f_i / 10.0),
            (f_i / 10.0) * (f_i / 10.0) * (f_i / 10.0)
        );
    }
}
