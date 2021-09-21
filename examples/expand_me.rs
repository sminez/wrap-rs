use wrap_rs::wrap_fn;

#[wrap_fn(bar => Result<String, u8>)]
fn foo(success: bool) -> &'static str {
    if success {
        "it works"
    } else {
        "it doesn't work"
    }
}

fn bar(s: &'static str) -> Result<String, u8> {
    println!("Wrapping '{}' returned from inner", s);

    if s == "it works" {
        Ok("it still works".into())
    } else {
        Err(5)
    }
}

fn main() {
    println!("{:?}", foo(true));
    println!("{:?}", foo(false));
}
