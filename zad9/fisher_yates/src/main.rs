use rand::{thread_rng, Rng};

// max permutation size
const N: usize = 1000;

fn main() {
    let seq: Vec<usize> = (0..N).collect();

    let mut rng = thread_rng();

    let mut permutation: Vec<usize> = seq.clone();

    for n in 1..=N {
        for _ in 0..(N*N) {
            permutation.splice(0..n, (&seq[0..n]).iter().cloned());
            //let mut permutation: Vec<usize> = (0..n).collect();
            for i in (1..n).rev() {
                let j: usize = rng.gen_range(0..=i);
                permutation.swap(i, j);
            }
            println!("{:?}", permutation);
        }
    }
}
