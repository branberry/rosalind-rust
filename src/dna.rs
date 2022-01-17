use std::collections::HashMap;
// Counting DNA Nucleotides: https://rosalind.info/problems/dna/
pub fn count_nucleotides(sequence: &str) -> HashMap<char, i32> {
    let mut counts: HashMap<char, i32> = HashMap::new();
    for nucleotide in sequence.chars() {
        let mut current_value = 0;
        if counts.contains_key(&nucleotide) {
            current_value = *(counts.get(&nucleotide).unwrap());
        }

        counts.insert(nucleotide, current_value + 1);
    }

    counts
}

// Transcribing DNA into RNA: https://rosalind.info/problems/rna/
pub fn dna_to_rna(dna_seq: &str) -> String {
    let mut rna_seq = String::new();

    for nucleotide in dna_seq.chars() {
        if nucleotide == 'T' {
            rna_seq.push('U');
        } else {
            rna_seq.push(nucleotide);
        }
    }

    rna_seq
}

// Complementing a Strand of DNA: https://rosalind.info/problems/revc/
pub fn dna_reverse_complement(dna_seq: &str) -> String {
    let mut rna_rev_comp = String::new();

    for nucleotide in dna_seq.chars().rev() {
        if nucleotide == 'T' {
            rna_rev_comp.push('A')
        } else if nucleotide == 'A' {
            rna_rev_comp.push('T');
        } else if nucleotide == 'C' {
            rna_rev_comp.push('G');
        } else if nucleotide == 'G' {
            rna_rev_comp.push('C');
        }
    }

    rna_rev_comp
}

/**
 *  Mendel's First Law: https://rosalind.info/problems/iprb/
 * k -> number or homozygous dominant factors
 * m -> num heterozygous factors
 * n -> num homozygous recessive factors
*/
pub fn mendels_first_law(k: i32, m: i32, n: i32) -> f64 {
    let mut prob_dominant = 0.0;
    let population = k + m + n;

    let prb_k = k as f64 / population as f64;

    prob_dominant += prb_k * ((k - 1) as f64 / (population - 1) as f64);
    prob_dominant += prb_k * (m as f64 / (population - 1) as f64);
    prob_dominant += prb_k * (n as f64 / (population - 1) as f64);

    let prb_m = m as f64 / population as f64;

    prob_dominant += prb_m * (k as f64 / (population - 1) as f64);
    prob_dominant += prb_m * ((m - 1) as f64 / (population - 1) as f64) * 0.75;
    prob_dominant += prb_m * (n as f64 / (population - 1) as f64) * 0.5;

    let prb_n = n as f64 / population as f64;

    prob_dominant += prb_n * (k as f64 / (population - 1) as f64);
    prob_dominant += prb_n * (m as f64 / (population - 1) as f64) * 0.5;

    prob_dominant
}

/**
 * Rabbits and Recurrence Relations: https://rosalind.info/problems/fib/
*/
pub fn recurrence_relations(n: i64, k: i64) -> i64 {
    let mut cache = vec![0, 1, 1];

    for idx in 3..(n + 1) {
        let f_n1 = *cache.get((idx - 1) as usize).unwrap();
        let f_n2 = *cache.get((idx - 2) as usize).unwrap();

        cache.insert(idx as usize, f_n1 + k * f_n2);
    }

    return *cache.get(n as usize).unwrap();
}
