fn another(x:i32) {
    println!("the value of x is {x}")
}

fn main() {
    // immutable
    let x = 10;
    println!("x is {}", x);
    // x = 20;  // error

    let mut a = 12;
    println!("a is {}", a);
    a = 15;
    println!("after a is {}", a);

    // data type converse
    let guess: u32 = "123".parse().expect("Not a number");
    println!("guess is {}", guess);

    // expect error: not a number: ParseIntError { kind: InvalidDigit }
    // let invalid_data:u32 = "a1234".parse().expect("not a number");
    // println!("invalid_data is {}", invalid_data);

    let tuple: (i32, i64, f32, u8) = (100, 52, 23.232423, 130);
    println!("tuple is {:?}", tuple);
    println!("tuple is {:#?}", tuple);
    println!("tuple is {:#?}", tuple.0);
    println!("tuple is {:#?}", tuple.1);
    println!("tuple is {:#?}", tuple.2);
    println!("tuple is {:#?}", tuple.3);

    // functions
    another(23);
}
