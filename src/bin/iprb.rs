fn main() {
    let inp: Vec<usize> = include_str!("iprb.txt")
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let res = probability_dominant(inp[0], inp[1], inp[2]);
    dbg!(res);
}

#[allow(non_snake_case)]
fn probability_dominant(AA: usize, Aa: usize, aa: usize) -> f64 {
    let (AA, Aa, aa) = (AA as f64, Aa as f64, aa as f64);
    let sum = AA + Aa + aa;
    let p_aaaa = aa / sum * (aa - 1.) / (sum - 1.);
    let p_AaAa = Aa / sum * (Aa - 1.) / (sum - 1.);
    let p_Aaaa = Aa / sum * aa / (sum - 1.) * 2.;
    1. - p_aaaa - p_AaAa * 0.25 - p_Aaaa * 0.5
}
