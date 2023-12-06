struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

fn use_struct() {
    let mut user1 = User {
        username: String::from("someone"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("new@example.com");
    println!(
        "user1: {} {} {} {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    let _ = build_user(String::from("email"), String::from("username"));

    let _user2 = User {
        active: false,
        ..user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", origin.0, origin.1, origin.2);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangles() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("recv1 is {:#?} !", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("{}", Rectangle::square(30).area());
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Self) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    use_struct();
    tuple_struct();
    rectangles();
}
