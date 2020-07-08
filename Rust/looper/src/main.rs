fn main() {
    println!("Hello, world!");
    loopto10();
    forloopto10();
    whileloopto10();
    array_loop();
}

fn loopto10(){
    let mut n = 0;
    loop {
        n += 1;
        println!("Hello");
        if n > 10 {
            return;
        }
    }
}

fn forloopto10() {
    for n in 0..10 {
        println!("Hello, {}!", n);
    }
}

fn whileloopto10() {
    let mut n = 0;
    while n < 10 {
        n += 1;
        println!("Hello while {}!", n)
    }
}

fn array_loop(){
    // let mut v = Vec::new();
    // v.push(4);
    // v.push(7);
    // v.push(9);

    let v = vec![4,7,8,9,10,11];

    'outer: for i in 0..10 {
        for n in &v {
            if i+n == 11 {
                break 'outer;
            }
            println!("{}", n);
        }
    }
}