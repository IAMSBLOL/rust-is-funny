use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    println!("Hello, world!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");

    let number: u32 = "123".parse().expect("fucking number");

    print!("parse is {number}");

    let r_tuple = (12, 12.3, "art");
    let (x, y, z) = r_tuple;
    print!("parse is {x}");
    print!("parse is {y}");
    print!("parse is {z}");
    print!("parse is {:?}", r_tuple);

    let arr = [1, 2, 3, 4];

    let index = 2;

    let ele = arr[index];
    print!("123 {ele}");

    test_mihoyo();
}

fn test_mihoyo() {
    let mut y = {
        let x = 3;
        if x > 2 {
            x + 1
        } else {
            x + 2
        }
    };

    println!("The value of y is: {y}\n");

    loop {
        if y > 10 {
            break;
        } else {
            y = y + 1;
        }
        print!("y is {y}\n")
    }

    {
        let test = "hello";
        let x = 5;
        let y = x;
        print!("{test}\n");
        print!("yyyy{y}\n")
    };
    let test = "hel1lo";
    print!("{test}\n")
}
