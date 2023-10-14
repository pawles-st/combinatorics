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

fn count_orbits(permutation: &[usize]) -> usize {
    let mut no_orbits = 0;
    
    let mut visited: Vec<bool> = vec![false; permutation.len()];
    for i in 0..permutation.len() {
        if !visited[i] {
            let start = i;
            let mut curr = i;
            loop {
                visited[curr] = true;
                curr = permutation[curr];
                if curr == start {
                    break;
                }
            }
            no_orbits += 1;
        }
    }
    return no_orbits;
}

fn main() {
    let seq: Vec<usize> = (0..N).collect();

    let mut rng = thread_rng();
    let uni = Uniform::new(0, 9);
    println!("i = {:?}", uni);
    let ranges: Vec<Uniform<usize>> = (1..=N).map(|val| Uniform::new(0, val + 1)).collect();

    let mut permutation: Vec<usize> = seq.clone();

    for n in 2..=N {
        let mut no_fixed = 0;
        let mut one_fixed = 0;
        let mut orbits = 0;
        for _ in 0..(N*N) {
            for i in (1..n).rev() {
                //let j = rng.sample(ranges[19]);
                //println!("{}", j);
                //println!("i = {}", i - 1);
                //let j: usize = rng.gen_range(0..=i);
                let j = rng.sample(ranges[i - 1]);
                //println!("{}", j);
                //println!("-----");
                permutation.swap(i, j);
            }
            //println!("{:?}", &permutation[0..n]);
            let fixed_points = count_fixed_points(&permutation[0..n]);

            if fixed_points == 0 {
                no_fixed += 1;
            } else if fixed_points == 1 {
                one_fixed += 1;
            }

            orbits += count_orbits(&permutation[0..n]);

            permutation.splice(0..n, (&seq[0..n]).iter().cloned());
        }
        println!("no - {}", no_fixed);
        println!("one - {}", one_fixed);
        println!("orbits - {}", orbits);
    }
}
