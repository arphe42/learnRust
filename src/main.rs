fn hahaha() -> String {
    return String::from("Hello World!");
}

fn two_times(x:i32) -> i32 {
    return x * 2;
}

fn comparaison(x:i32) -> bool {
    return x == 2;
}

fn main() {
    println!("{}", hahaha());

    let mut x: i32 = 0;
    while x <= 20 {
        println!("{}", two_times(x));
        x += 1;
    }

    for n in 1..=15 {
        println!("{}", n);
    }

    println!("{}", comparaison(2));
}
