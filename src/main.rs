use rubigino::display::font::{Characters};

fn main() {
    let input: String = format!(" { } ", "Hello");
    let chars: Characters = Characters::from(input);

    println!("{}", chars);

    /*for c in chars {
        println!("{}", c >> 4);
    }*/
}
