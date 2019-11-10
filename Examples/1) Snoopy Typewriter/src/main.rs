fn snoopy_write() -> String {
    let text : String = "".to_string();
    text.push_str("It was a dark and stormy night");
    text
}

fn main() {
    let snoopy_text = snoopy_write();
    println!( "Snoopy writes: \"{}\"", snoopy_text );
}
