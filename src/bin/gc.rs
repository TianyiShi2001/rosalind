//! http://rosalind.info/problems/gc/

fn main() {
    let inp = rosalind::utils::read_fasta(include_str!("gc.fa"));
    let mut max_id = "";
    let mut max_gc = 0f64;
    for (id, dna) in inp {
        dbg!((id, &dna));
        let curr_gc = gc(&dna);
        if curr_gc > max_gc {
            max_id = id;
            max_gc = curr_gc;
        }
    }
    dbg!((max_id, max_gc * 100.));
}

fn gc(inp: &str) -> f64 {
    inp.chars().fold(0, |count, c| {
        count + if c == 'C' || c == 'G' { 1 } else { 0 }
    }) as f64
        / inp.len() as f64
}
