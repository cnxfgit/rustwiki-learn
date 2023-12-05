const MAX_POINTS: u32 = 100_000;

fn variable() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn shadow() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

fn unicode() {
    let cry = '😂';
    println!("The value of cry is: {}", cry);
}

fn tuple() {
    let tup = (1, 1.2, "hello");
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);
}

fn array() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The array of arr is: {:?}", arr);
    let arr = [3; 5];
    println!("The array of [3; 5] is: {:?}", arr);

    let a = [1, 2, 3, 4, 5];
    let index = 4;
    let element = a[index];
    println!("The value of element is: {}", element);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn if_statement() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}

fn loop_func() {
    let mut n = 10;
    loop {
        println!("again!");
        n -= 1;
        if n <= 0 {
            break;
        }
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    while n < 10 {
        n += 1;
        println!("while again!");
    }

    let a = [1, 2, 3, 4, 5];
    let mut idx = 0;
    while idx < a.len() {
        println!("the value is: {}", a[idx]);
        idx += 1;
    }

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("the number is: {}", number);
    }
}

fn main() {
    variable();
    shadow();
    unicode();
    tuple();
    array();

    another_function(666);
    let _ = five();
    if_statement();
    loop_func();
}
