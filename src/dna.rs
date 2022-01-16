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
pub fn dna_rna_reverse_complement(dna_seq: &str) -> String {
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
