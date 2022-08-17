use std::fmt::Error;

fn te() -> fn(d: &str) -> Result<&str, ()> {
    let f = move |c: &str| -> Result<&str, ()> {
        println!("{}", c);
        Ok("sdf")
    };
    f
}

fn main() {
    te()("sdfds");
}
