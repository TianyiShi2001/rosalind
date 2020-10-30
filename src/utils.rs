pub mod fasta;
pub use fasta::read_fasta;
use std::str::FromStr;

// fn args<T>(s: &'static str, f: impl Fn(&str) -> T) -> impl IntoIterator<Item = T> {
//     s.split_whitespace().map(f)
// }

pub fn parse_args<T>(s: &'static str) -> impl Iterator<Item = T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    s.split_whitespace().map(|x| x.parse::<T>().unwrap())
}
