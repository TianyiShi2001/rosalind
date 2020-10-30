fn main() {
    let inp = include_str!("hamm.txt")
        .trim_end()
        .split(char::is_whitespace)
        .collect::<Vec<_>>();
    let res = hamming_naive(inp[0], inp[1]);
    dbg!(res);
}

fn hamming_naive(x: &str, y: &str) -> usize {
    assert_eq!(
        x.len(),
        y.len(),
        "The lengths of the two sequences must equal."
    );

    x.chars().zip(y.chars()).filter(|(p, q)| p != q).count()
}
