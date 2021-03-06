mod dna;
#[cfg(test)]
mod tests {
    use crate::dna::{self, mendels_first_law, recurrence_relations};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn test_count_nucleotides() {
        let dna_seq: &str = "ACTGACTGACTGACTG";
        let result = dna::count_nucleotides(dna_seq);

        assert_eq!(result.get(&'A').unwrap().to_owned(), 4);
        assert_eq!(result.get(&'C').unwrap().to_owned(), 4);
        assert_eq!(result.get(&'T').unwrap().to_owned(), 4);
        assert_eq!(result.get(&'G').unwrap().to_owned(), 4);
    }

    #[test]
    fn test_dna_to_rna() {
        let dna_seq: &str = "AATTCGGTTT";
        let result = dna::dna_to_rna(dna_seq);

        assert_eq!(result, "AAUUCGGUUU");
    }

    #[test]
    fn test_dna_reverse_complement() {
        let dna_seq = "AAAACCCGGT";

        let result = dna::dna_reverse_complement(dna_seq);

        assert_eq!(result, "ACCGGGTTTT");

        let rosalind_test = "TAATTCATAGGCTGTAGGCTCCGCCTGCTGGACTATACGTATATACATGAGCCAACAGAATAACTTCTACCGCAATCCCTCAGATGTTGCGGCCAGGCATATACATGTAACCCCTTCAGGCACGACTTGAATCCGGATCACGCTGTAATTATAGGCGTCTAGATCGTTTGAAGGTATAGAGATAATGAGATGACTTCACGCTTTAAGAGCCACACCGCCGTCCAACTATCGGCTGTGTGTCTGTAAGATTTCACGCAGGCGCATTAAATATACTTCACGGTAGACTGGCAGGATCTTTAGTGAGGCGGATTATCACTGAGGCGCCAAACAGCGACTGTGAACAAGCTTGACCGATGATCGAAGTATTTGATATCGAAGTATCAAAGCGTGAACCCACGCTGTCCTTATGGTGCTCGTTACATACTCGCAGTCTCGAGACATATAGACAGGTCAAGGGGTGGAACTACATATCATAACTTTCATTATCCTCGGGCGAAAAGATGGTAGGTCCGGCCGGCGTTGGCTCTGCGTTTACGCAATCAACCCTGAGAGGCAAAAGCGAAATGGATATTAAAGAGTGTCAAATGTTTGTCTTCAATAGCCATTTTAAGGTTCAAACGACAATTTGCCGATACGTAACGCTTCTAGAGGTTGATCTAACGCATCACGGGGGCAGCCGTCTTGCGGCGCGGAACCCAGCGATCATAAGGAGCCGATACGAGACGTCGGTGGAGTTTGTGATGAAGATCAGTTCTGCCACAAGTTGCCTGTTTTTCCTGGTGTCTGCGTCCAGTTGCTGACAGGCTAAAATTCATGTATGATCGCGTGCAGAACGGAGGAACCATAAAACCTGGTTAGCAGAGAAATAGCACTAGATAGTTGGGGGTCTCTAATCTCTCTCAAGGGATTCCACGCTCAAGTATAAAGTTGTAATTATGTGCAAAAGCCCCAACGCAGCGATAAGACAACCGATTAGG";

        let rosalind_result = dna::dna_reverse_complement(rosalind_test);

        println!("{}", rosalind_result);
    }

    #[test]
    fn test_mendels_first_law() {
        let result = mendels_first_law(30, 20, 18);

        assert_eq!("0.83297", format!("{:.5}", result));
    }

    #[test]
    fn test_recurrence_relations() {
        let result = recurrence_relations(33, 2);

        assert_eq!(result, 2863311531);
    }
}
