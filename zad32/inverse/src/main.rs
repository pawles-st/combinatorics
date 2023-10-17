use std::env;
use std::io;
use std::io::Write;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        std::process::exit(1);
    }

    let no_coefficients: usize = args[1].parse().expect("number of coefficents should be a number");

    let mut buf: String = Default::default();
    
    let mut coefficients: Vec<f64> = Vec::new();
    for i in 0..no_coefficients {
        buf.clear();
        print!("input coefficient nr {}: ", i);
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buf).expect("can't read the input");
        coefficients.push(1.0 / buf.trim().parse::<f64>().expect("coefficient should be a number"));
    }

    let inverse_c0 = 1.0 / coefficients[0];
    let mut inverse_coefficients = vec![inverse_c0];

    for k in 1..no_coefficients {
        let mut s = 0.0;
        for i in 1..=k {
            s += coefficients[i] * inverse_coefficients[k - i];
        }
        inverse_coefficients.push(-inverse_c0 * s);
    }

    println!("{:?}", inverse_coefficients);
}
