fn test_loop_rev() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn test_loop() {
    // 1, 2, 3
    for number in 1..4 {
        println!("{}!", number);
    }
}

fn test_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}

fn test_iter() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn test_return_loop() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

fn main() {
    test_loop();
    test_loop_rev();
    test_while();
    test_iter();
    test_return_loop();
}

