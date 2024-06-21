enum Div {
    Ok(f64),
    DivZero,
}

fn div(a: f64, b: f64) -> Div {
    if b == 0.0 {
        return Div::DivZero;
    }
    return Div::Ok(a/b);
}

fn main() {
    match div(5.0, 0.0) {
        Div::DivZero => eprintln!("ゼロ割ですよ！"),
        Div::Ok(ans) => println!("こたえ: {}", &ans),
    }

    match div(5.0, 2.0) {
        Div::DivZero => eprintln!("ゼロ割ですよ！"),
        Div::Ok(ans) => println!("こたえ: {}", &ans),
    }
}
