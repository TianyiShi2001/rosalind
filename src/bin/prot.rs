fn main() {
    let inp: &'static str = include_str!("prot.txt").trim_end();
    let res = translate(inp);
    dbg!(res);
}

fn translate(mrna: &str) -> String {
    let mut prot = String::new();
    for code in mrna.as_bytes().chunks_exact(3) {
        match code {
            // TODO: use '|'
            b"UUU" => prot.push('F'),
            b"CUU" => prot.push('L'),
            b"AUU" => prot.push('I'),
            b"GUU" => prot.push('V'),
            b"UUC" => prot.push('F'),
            b"CUC" => prot.push('L'),
            b"AUC" => prot.push('I'),
            b"GUC" => prot.push('V'),
            b"UUA" => prot.push('L'),
            b"CUA" => prot.push('L'),
            b"AUA" => prot.push('I'),
            b"GUA" => prot.push('V'),
            b"UUG" => prot.push('L'),
            b"CUG" => prot.push('L'),
            b"AUG" => prot.push('M'),
            b"GUG" => prot.push('V'),
            b"UCU" => prot.push('S'),
            b"CCU" => prot.push('P'),
            b"ACU" => prot.push('T'),
            b"GCU" => prot.push('A'),
            b"UCC" => prot.push('S'),
            b"CCC" => prot.push('P'),
            b"ACC" => prot.push('T'),
            b"GCC" => prot.push('A'),
            b"UCA" => prot.push('S'),
            b"CCA" => prot.push('P'),
            b"ACA" => prot.push('T'),
            b"GCA" => prot.push('A'),
            b"UCG" => prot.push('S'),
            b"CCG" => prot.push('P'),
            b"ACG" => prot.push('T'),
            b"GCG" => prot.push('A'),
            b"UAU" => prot.push('Y'),
            b"CAU" => prot.push('H'),
            b"AAU" => prot.push('N'),
            b"GAU" => prot.push('D'),
            b"UAC" => prot.push('Y'),
            b"CAC" => prot.push('H'),
            b"AAC" => prot.push('N'),
            b"GAC" => prot.push('D'),
            b"UAA" => break,
            b"CAA" => prot.push('Q'),
            b"AAA" => prot.push('K'),
            b"GAA" => prot.push('E'),
            b"UAG" => break,
            b"CAG" => prot.push('Q'),
            b"AAG" => prot.push('K'),
            b"GAG" => prot.push('E'),
            b"UGU" => prot.push('C'),
            b"CGU" => prot.push('R'),
            b"AGU" => prot.push('S'),
            b"GGU" => prot.push('G'),
            b"UGC" => prot.push('C'),
            b"CGC" => prot.push('R'),
            b"AGC" => prot.push('S'),
            b"GGC" => prot.push('G'),
            b"UGA" => break,
            b"CGA" => prot.push('R'),
            b"AGA" => prot.push('R'),
            b"GGA" => prot.push('G'),
            b"UGG" => prot.push('W'),
            b"CGG" => prot.push('R'),
            b"AGG" => prot.push('R'),
            b"GGG" => prot.push('G'),
            _ => unreachable!(),
        }
    }
    prot
}