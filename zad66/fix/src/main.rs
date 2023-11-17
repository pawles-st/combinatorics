use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

// max permutation size
const N: usize = 1000;

fn count_fixed_points(permutation: &[usize]) -> usize {
    let mut fixed_pts = 0;
    for i in 0..permutation.len() {
        if permutation[i] == i {
            fixed_pts += 1;
        }
    }
    return fixed_pts;
}

fn main() {
    let seq: Vec<usize> = (0..N).collect();

    let mut rng = thread_rng();
    let ranges: Vec<Uniform<usize>> = (1..=N).map(|val| Uniform::new(0, val + 1)).collect();

    let mut permutation: Vec<usize> = seq.clone();

    for n in 2..=N {
        let mut fixed = 0;
        let reps = N as f64;
        for _ in 0..(reps as usize) {
            for i in (1..n).rev() {
                let j = rng.sample(ranges[i - 1]);
                permutation.swap(i, j);
            }
            fixed += count_fixed_points(&permutation[0..n]);

            permutation.splice(0..n, (&seq[0..n]).iter().cloned());
        }
        println!("{}", fixed as f64 / reps);
    }
}
