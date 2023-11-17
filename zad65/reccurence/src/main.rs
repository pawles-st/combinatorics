fn main() {
    let mut factorial_inverse = 1.0;
    let mut n = 1;
    let mut prev = 0.0;
    let mut partial_sum = 0.0;

    loop {
        partial_sum += factorial_inverse * factorial_inverse;
        if partial_sum == prev {
            break;
        } else {
            println!("{}", partial_sum);
            prev = partial_sum;
            factorial_inverse *= 1.0 / n as f64;
            n += 1;
        }
    }
    println!("finished at n = {}", n);
}
