fn main() {
    let inp = rosalind::utils::read_fasta(include_str!("lcsm.txt"))
        .map(|x| x.1)
        .collect::<Vec<_>>();
    let lcs = cs::longest_common_substring(&inp);
    println!("{}", lcs);
}
