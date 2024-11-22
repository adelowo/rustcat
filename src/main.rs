use fake::{faker::lorem::en::Sentences, Fake};

fn main() {
    let sentences: Vec<String> = Sentences(5..20).fake();

    let text = sentences.join("; ").replace('\n', "");

    let words = text.split(' ');

    for (idx, t) in words.enumerate() {
        let (r, g, b) = rgb(idx as i32);
        print!("\x1b[38;2;{};{};{}m{} \x1b[0m", r, g, b, t);
    }

    println!()
}

fn rgb(i: i32) -> (i32, i32, i32) {
    let f = 0.1;
    let red = (f64::sin(f * i as f64 + 0.0) * 127.0 + 128.0) as i32;
    let green = (f64::sin(f * i as f64 + 2.0 * std::f64::consts::PI / 3.0) * 127.0 + 128.0) as i32;
    let blue = (f64::sin(f * i as f64 + 4.0 * std::f64::consts::PI / 3.0) * 127.0 + 128.0) as i32;
    (red, green, blue)
}
