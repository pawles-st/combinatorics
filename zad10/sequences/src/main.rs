use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

// max sequence length
const N: usize = 50;

fn main() {

    let mut rng = thread_rng();
    let dist = Uniform::new(0, 2);

    for n in 1..=N {
        for _ in 0..(N * N) {
            let mut sequence = String::new();
            sequence.reserve(n);
            for _ in 0..n {
                sequence.push(char::from_u32(0x0041 + rng.sample(dist)).expect("can't convert to a char value"));
            }
            println!("{}", sequence);
        }    
    }
}
