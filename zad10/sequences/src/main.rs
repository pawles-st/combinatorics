use rand::{thread_rng, Rng};
use rand::distributions::Uniform;

// max sequence length
const N: usize = 50;

fn create_lps(pattern: &str) -> Vec<usize> {
    let mut lps = Vec::new();

    let pattern_chars: Vec<char> = pattern.chars().collect();
    lps.reserve(pattern_chars.len());

    let mut k = 0;
    lps.push(0);
    for q in 1..pattern_chars.len() {
        while k > 0 && pattern_chars[k] != pattern_chars[q] {
            k = lps[k - 1];
        }
        if pattern_chars[k] == pattern_chars[q] {
            k += 1;
        }
        lps.push(k);
    }

    return lps;
}

fn find(lps: &Vec<usize>, pattern: &str, text: &str) -> usize {
    let mut no_occurences = 0;

    let pattern_chars: Vec<char> = pattern.chars().collect();
    let mut text_chars = text.chars();

    let mut q = 0;
    for _ in 0..text.chars().count() {
        let c = text_chars.next().unwrap();
        while q > 0 && pattern_chars[q] != c {
            q = lps[q - 1];
        }
        if pattern_chars[q] == c {
            q += 1;
        }
        if q == pattern_chars.len() {
            no_occurences += 1;
            q = lps[q - 1];
        }
    }
    return no_occurences;
}

fn main() {

    let mut rng = thread_rng();
    let dist = Uniform::new(0, 2);

    let lps_aaa = create_lps("aaa");
    let lps_abb = create_lps("abb");

    let repetitions = N * N * N;

    for n in 1..=N {
        let mut aaa_sequences = 0;
        let mut abb_sequences = 0;
        let mut total_aaa_occurences = 0;
        for _ in 0..repetitions {
            let mut sequence = String::new();
            sequence.reserve(n);
            for _ in 0..n {
                sequence.push(char::from_u32(0x0061 + rng.sample(dist)).expect("can't convert to a char value"));
            }
            let aaa_occurences = find(&lps_aaa, "aaa", &sequence);
            let abb_occurences = find(&lps_abb, "abb", &sequence);
            if aaa_occurences > 0 {
                aaa_sequences += 1;
                total_aaa_occurences += aaa_occurences;
            }
            if abb_occurences > 0 {
                abb_sequences += 1;
            }
        }
        println!("{}", n);
        println!("no_aaa: {}", aaa_sequences as f64 / repetitions as f64);
        println!("no_abb: {}", abb_sequences as f64 / repetitions as f64);
        println!("total_aaa: {}", total_aaa_occurences as f64 / repetitions as f64);
    }
}
