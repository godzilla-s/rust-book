// 数组、以及数组结构

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// tuple struct

struct Color(i32, i32, i32);

struct Rectangle(i32, i32);

struct Rect {
    width: i32,
    height: i32,
}

fn main() {
    let u = User{
        email: String::from("test@hotmail.com"),
        username: String::from("test"),
        sign_in_count: 1,
        active: true,
    };

    println!("user: {}, {}, {}, {}", u.username, u.email, u.sign_in_count, u.active);

    // u.username = String::from("test2"); // error

    let mut u1 = User{
        email: String::from("test2@hotmail.com"),
        username: String::from("test2"),
        sign_in_count: 2,
        active: true,
    };
    u1.username = String::from("test002"); // OK

    let _c1 = Color(100, 0,100);
    let _c2 = Color(50, 50, 0); // _c2.0  _c2.1 _c2.2
    let _c3 = _c2;
    println!("({}, {}, {})", _c3.0, _c3.1, _c3.2);

    let r1 = Rectangle(10, 20);
    println!("area is {}", area(r1));
    // println!("rectangle: width: {}, height: {}", r1.0, r1.1) // error: can't borrow

    let _r = Rect{width: 100, height:200};
}


fn area(rect:Rectangle) ->i32 {
    rect.0 * rect.1
}