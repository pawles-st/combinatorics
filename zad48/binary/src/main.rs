const N: usize = 1024 * 1024;

fn sd(n: usize) -> usize {
    return (0..=n.ilog2()).scan(1, |bit_value, _| {
        let val = *bit_value;
        *bit_value *= 2;
        return Some(val);
    }).filter(|val| n & val != 0).count();
}

fn main() {
    let mut s = 0;
    for n in 1..=N {
        s += sd(n);
        println!("{};{}", n, s)
    }
}
