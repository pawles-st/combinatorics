use rug::Integer;

const N: usize = 10000;

fn compute_divisors_sum(n: usize) -> Vec<Integer> {
    let mut divisor_sum = vec![Integer::from(0); n + 1];

    for divisor in 1..=n {
        let mut multiply = divisor;
        while multiply <= n {
            divisor_sum[multiply] += Integer::from(divisor);
            multiply += divisor;
        }
    }

    return divisor_sum;
}

fn main() {
    let divisor_sum = compute_divisors_sum(N);
    let mut p = vec![Integer::from(1); N + 1];
    //println!("0: {}", p[0]);
    println!("{}", p[0]);
    for n in 1..=N {
        p[n] = (1..=n).fold(Integer::from(0), |acc, j| acc + &divisor_sum[j] * &p[n - j]) / n;
        //println!("{}: {}", n, p[n]);
        println!("{}", p[n]);
    }
}
