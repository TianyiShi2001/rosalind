//! http://rosalind.info/problems/revc/

fn main() {
    let inp = include_str!("revc.txt").trim_end();
    let res = reverse_complement(inp);
    dbg!(res);
}

fn reverse_complement(inp: &str) -> String {
    inp.chars().rev().fold(String::new(), |mut s, c| {
        s.push(match c {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => unreachable!(),
        });
        s
    })
}
