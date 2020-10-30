fn main() {
    let inp = include_str!("dna.txt").trim_end();
    let res = count_dna_nucleotide(inp);
    println!("{} {} {} {}", res[0], res[1], res[2], res[3]);
}

/// Given: A DNA string s of length at most 1000 nt.
/// Return: Four integers (separated by spaces) counting the respective number of times that the symbols 'A', 'C', 'G', and 'T' occur in s.
fn count_dna_nucleotide(s: &str) -> [usize; 4] {
    s.chars().fold([0usize; 4], |mut acgt_count, nucleotide| {
        match nucleotide {
            'A' => acgt_count[0] += 1,
            'C' => acgt_count[1] += 1,
            'G' => acgt_count[2] += 1,
            'T' => acgt_count[3] += 1,
            _ => unreachable!(),
        }
        acgt_count
    })
}
