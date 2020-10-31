fn main() {
    let mut args = rosalind::utils::parse_args::<String>(include_str!("subs.txt"));
    let s = args.next().unwrap();
    let pattern = args.next().unwrap();
    let matches = Match::new(&s, &pattern);
    for i in matches {
        print!("{} ", i);
    }
}

// fn substr<'a>(s: &'a str, pattern: &'a str) -> impl Iterator<Item = usize> + 'a {
//     s.as_bytes()
//         .windows(pattern.len())
//         .enumerate()
//         .filter(|(idx, substr)| substr == &pattern.as_bytes())
//         .map(|(idx, _)| idx)
// }
struct Match<'a, 'b> {
    windows: std::slice::Windows<'a, u8>,
    pattern: &'b [u8],
    i: usize,
}

impl<'a, 'b> Match<'a, 'b> {
    fn new(s: &'a str, pattern: &'b str) -> Self {
        Self {
            windows: s.as_bytes().windows(pattern.len()),
            pattern: pattern.as_bytes(),
            i: 0,
        }
    }
}

impl<'a, 'b> Iterator for Match<'a, 'b> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(substr) = self.windows.next() {
            self.i += 1;
            if substr == self.pattern {
                return Some(self.i);
            }
        }
        None
    }
}
