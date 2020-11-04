fn maj(ints: &[u16]) -> Option<u16> {
    let mut counter = 0;
    let mut maj = ints[0];
    for &int in &ints[1..] {
        if counter == 0 {
            maj = int;
        }
        counter += if int == maj { 1 } else { -1 };
    }
    if ints.iter().filter(|&&x| x == maj).count() * 2 <= ints.len() {
        None
    } else {
        Some(maj)
    }
}

fn main() {
    let inp = include_str!("maj.txt")
        .split('\n')
        .skip(1)
        .map(|s| rosalind::utils::parse_args::<u16>(s).collect::<Vec<u16>>());

    for v in inp {
        match maj(&v) {
            None => print!("{}", -1),
            Some(i) => print!("{}", i),
        }
        print!(" ")
    }
}
