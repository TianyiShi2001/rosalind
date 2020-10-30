fn main() {
    let inp = include_str!("rna.txt").trim_end();
    let res = dna_to_rna(inp);
    dbg!(res);
}

fn dna_to_rna(dna: &str) -> String {
    dna.chars().fold(String::new(), |mut rna, nucleotide| {
        rna.push(if nucleotide == 'T' { 'U' } else { nucleotide });
        rna
    })
}
