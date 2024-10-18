// 引用与借用

fn main() {
    let mut s = String::from("hello world");

    let s_len = calculator_length(&s);

    println!("{} length is: {}", s, s_len);

    apend_str(&mut s);
    println!("after append s is: {}", s);

    let s2 = String::from("what a fuck");
    append_str2(&mut s, &s2);
    println!("append str: {}", s);

    let a = String::from("value 1");
    let a1 = &a;
    let a2 = &a;
    println!("a1 is {}, a2 is {}", a1, a2);

    let mut b = String::from("value 2");
    let b1 = &mut b;
    // let b2 = &mut b;
    println!("b1 is {}", b1);

    // string slice

    let c = String::from("hello world");
    let c1 = &c[0..3];
    println!("c1 is: {}", c1);
    let c2 = &c[1..4];
    println!("c2 is: {}", c2);
    println!("c3 is {}", &c[..4]);
}

// 借用
fn calculator_length(s: &String) -> usize {
    s.len()
}

fn apend_str(s:&mut String) {
    s.push_str("aaa");
}

fn append_str2(s:&mut String, s1:&String) {
    s.push_str(s1);
}