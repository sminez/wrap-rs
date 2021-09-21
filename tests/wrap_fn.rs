use wrap_rs::{wrap_fn, wrap_fn_async};

fn plus_1(n: usize) -> usize {
    n + 1
}

#[test]
fn same_return_type_no_args() {
    #[wrap_fn(plus_1 => usize)]
    fn f() -> usize {
        4
    }

    assert_eq!(f(), 5);
}

#[test]
fn same_return_type_with_args() {
    #[wrap_fn(plus_1 => usize)]
    fn f(b: bool) -> usize {
        if b {
            4
        } else {
            5
        }
    }

    assert_eq!(f(true), 5);
    assert_eq!(f(false), 6);
}

#[test]
fn original_return_is_unit() {
    const CALLED: &str = "called the wrapper";

    fn unit_to_string(_: ()) -> String {
        CALLED.into()
    }

    #[wrap_fn(unit_to_string => String)]
    fn noop() -> () {}

    assert_eq!(noop(), CALLED.to_string());
}

#[test]
fn changed_return_type() {
    fn ok_if_gt_5(n: usize) -> Result<bool, bool> {
        if n > 5 {
            Ok(true)
        } else {
            Err(false)
        }
    }

    #[wrap_fn(ok_if_gt_5 => Result<bool, bool>)]
    fn _4() -> usize {
        4
    }

    #[wrap_fn(ok_if_gt_5 => Result<bool, bool>)]
    fn _6() -> usize {
        6
    }

    assert_eq!(_4().unwrap_err(), false);
    assert_eq!(_6().unwrap(), true);
}

#[test]
fn generic_wrapper() {
    fn stringify(t: impl ToString) -> String {
        t.to_string()
    }

    #[wrap_fn(stringify => String)]
    fn num() -> usize {
        42
    }

    #[wrap_fn(stringify => String)]
    fn boolean() -> bool {
        true
    }

    assert_eq!(num(), "42".to_string());
    assert_eq!(boolean(), "true".to_string());
}

fn describe(n: usize) -> String {
    format!("The answer is {}", n)
}

#[test]
fn multiple_args() {
    #[wrap_fn(describe => String)]
    fn add(a: usize, b: usize) -> usize {
        a + b
    }

    assert_eq!(add(4, 5), "The answer is 9".to_string())
}

#[tokio::test]
async fn async_wrapped() {
    #[wrap_fn(describe => String)]
    async fn add_async(a: usize, b: usize) -> usize {
        a + b
    }

    assert_eq!(add_async(4, 5).await, "The answer is 9".to_string())
}

async fn async_describe(n: usize) -> String {
    format!("The answer is {}", n)
}

#[tokio::test]
async fn async_wrapper() {
    #[wrap_fn_async(async_describe => String)]
    async fn add_async(a: usize, b: usize) -> usize {
        a + b
    }

    assert_eq!(add_async(4, 5).await, "The answer is 9".to_string())
}
