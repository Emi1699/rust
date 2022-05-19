#![allow(dead_code)]
#[warn(unused_variables)]

fn main() {
    let condition = false;
    let _var: u32 = if condition {3} else {4};

    // println!("The number is {}", var);

    // wheel();

    // loop_label();

    // break_return();

    // whilee();

    // for_loop();

    reverse_list(8);

}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}

fn wheel() {
    loop {
        println!("weeee");
    }
}

fn reverse_list(x: i32) {
    for number in (1..x+1).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn loop_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}

fn break_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

fn whilee() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");
}