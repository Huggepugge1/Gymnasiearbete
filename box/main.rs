struct Fun {
    hello: i32
}

fn main() {
    let mut num: Box<Fun> = Box::new(Fun{hello: 59});
    num.hello += 10;
    println!("{}", ((1 as u64) << 60));
}
