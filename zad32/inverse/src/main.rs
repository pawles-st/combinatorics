use std::env;

fn calculate_inverse(f: &dyn Fn(f64) -> f64, no_coefficients: usize) -> Vec<f64> {
    let inverse_c0 = 1.0 / f(0.0);
    let mut inverse_coefficients = vec![inverse_c0];

    for k in 1..no_coefficients {
        let mut s = 0.0;
        for i in 1..=k {
            s += f(i as f64) * inverse_coefficients[k - i];
        }
        inverse_coefficients.push(-inverse_c0 * s);
    }

    return inverse_coefficients;
}

fn one(_x: f64) -> f64 {
    return 1.0;
}

fn two_n(x: f64) -> f64 {
    return f64::powf(2.0, x);
}

fn fact(x: f64) -> f64 {
    let mut r = 1.0;
    for i in 2..=(x as usize) {
        r *= i as f64;
    }
    return r;
}

fn invfact(x: f64) -> f64 {
    return 1.0 / fact(x);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    /*
    if args.len() < 2 {
        std::process::exit(1);
    }

    let invert_coefficients = if args.len() > 2 {args[2] == "1"} else {false};
    */

    let no_coefficients: usize = args[1].parse().expect("number of coefficents should be a number");

    /*
    let mut buf: String = Default::default();
   
    let mut coefficients: Vec<f64> = Vec::new();
    for i in 0..no_coefficients {
        buf.clear();
        print!("input coefficient nr {}: ", i);
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut buf).expect("can't read the input");
        if invert_coefficients {
            coefficients.push(1.0 / buf.trim().parse::<f64>().expect("coefficient should be a number"));
        } else {
            coefficients.push(buf.trim().parse::<f64>().expect("coefficient should be a number"));
        }
    }
    */

    println!("{:?}", calculate_inverse(&one, no_coefficients));
    println!("{:?}", calculate_inverse(&two_n, no_coefficients));
    println!("{:?}", calculate_inverse(&fact, no_coefficients));
    println!("{:?}", calculate_inverse(&invfact, no_coefficients));
}
