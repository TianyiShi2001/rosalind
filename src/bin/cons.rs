fn main() {
    let inp = rosalind::utils::read_fasta_seq(include_str!("cons.txt"));
    let mut cons = Concensus::default();
    for dna in inp {
        cons.add_dna(&dna);
    }
    let consensus_sequence: String = (&cons).into();
    println!("{}", &consensus_sequence);
    println!("{}", cons);
}

#[derive(Default)]
struct Concensus {
    a: Vec<usize>,
    c: Vec<usize>,
    g: Vec<usize>,
    t: Vec<usize>,
}

impl Concensus {
    pub fn add_dna(&mut self, seq: &str) {
        if self.a.is_empty() {
            self.resize(seq.len());
        }
        seq.chars().enumerate().for_each(|(idx, c)| match c {
            'A' => self.a[idx] += 1,
            'C' => self.c[idx] += 1,
            'G' => self.g[idx] += 1,
            'T' => self.t[idx] += 1,
            _ => unreachable!(),
        })
    }
    pub fn resize(&mut self, len: usize) {
        self.a.resize(len, 0);
        self.c.resize(len, 0);
        self.g.resize(len, 0);
        self.t.resize(len, 0);
    }
}

impl From<&Concensus> for String {
    fn from(inp: &Concensus) -> String {
        let len = inp.a.len();
        let mut res = String::with_capacity(len);
        for i in 0..len {
            let mut nuc = 'A';
            let mut max_freq = inp.a[i];
            if inp.c[i] > max_freq {
                nuc = 'C';
                max_freq = inp.c[i];
            }
            if inp.g[i] > max_freq {
                nuc = 'G';
                max_freq = inp.g[i];
            }
            if inp.t[i] > max_freq {
                nuc = 'T';
                // max_freq = inp.t[i];
            }
            res.push(nuc)
        }
        res
    }
}

use std::fmt;

impl fmt::Display for Concensus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut write_oneline = |v: &Vec<usize>, c: char| {
            write!(
                f,
                "{}: {}\n",
                c,
                v.iter()
                    .map(|v| format!("{}", *v))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
        };
        write_oneline(&self.a, 'A')?;
        write_oneline(&self.c, 'C')?;
        write_oneline(&self.g, 'G')?;
        write_oneline(&self.t, 'T')?;
        Ok(())
    }
}
