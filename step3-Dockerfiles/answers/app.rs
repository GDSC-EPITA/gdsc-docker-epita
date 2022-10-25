use std::env;

fn main() {
    match env::var("LANG") {
        Ok(lang) => {
            if lang.eq(&String::from("EN")) {
                println!("Hello World");
            } else {
                println!("Bonjour Monde");
            }
        }
        Err(e) => println!("Couldn't read LANG ({})", e),
    };
}
