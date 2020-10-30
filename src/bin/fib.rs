fn main() {
    let mut args = rosalind::utils::parse_args::<u32>(include_str!("fib.txt"));
    let n = args.next().unwrap() - 1;
    let k = args.next().unwrap();
    let res = FibRabbit::new(k).nth(n as usize).unwrap();
    dbg!(res);
}

struct FibRabbit {
    n_2: u32,
    n_1: u32,
    k: u32,
}

impl FibRabbit {
    pub fn new(k: u32) -> Self {
        Self { n_2: 1, n_1: 1, k }
    }
}

impl Iterator for FibRabbit {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        let tmp = self.n_2 * self.k + self.n_1;
        // dbg!(tmp);
        Some(std::mem::replace(
            &mut self.n_2,
            std::mem::replace(&mut self.n_1, tmp),
        ))
    }
}
