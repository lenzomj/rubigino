use rubigino::display::font::Character;

fn main() {
    let input: String = format!(" { } ", "Hello");
    let raw_chars: Vec<char>  = input.chars().rev().collect();
    let chars: Vec<Character> = raw_chars
        .iter()
        .map(|c| Character::from(*c))
        .collect();

    for window in chars.windows(2) {
        println!("{}", window[0]);
    }
}



