//! binary search

fn main() {
    let mut args = include_str!("bins.txt")
        .split("\n")
        .skip(2)
        .map(rosalind::utils::parse_args::<i32>);
    let v = args.next().unwrap().collect::<Vec<i32>>();
    let searches = args.next().unwrap();
    searches.for_each(|q| match v.binary_search(&q) {
        Err(_) => print!("{} ", -1),
        Ok(idx) => print!("{} ", idx + 1),
    });
}
