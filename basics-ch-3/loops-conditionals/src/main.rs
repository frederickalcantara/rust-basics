fn main() {
    // let number = 6;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("Condition was false");
    // }

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // let condition = false;
    // let num = if condition { 5 } else { 6 };

    // println!("The value of num is: {}", num);

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count: {}", count);

    //     let mut remaining = 10;

    //     loop {
    //         println!("remining = {}", remaining);
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {}", count);

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);

    // let mut while_num = 3;

    // while while_num != 0 {
    //     println!("{}!", while_num);

    //     while_num -= 1;
    // }

    // println!("LIFTOFF!!!");

    // let arr = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the arr value is: {}", arr[index]);

    //     index += 1;
    // }

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the a value is: {}", element);
    // }

    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    // temp = 32;

    let temp = 32.0;

    convert_temp(temp, false);


    let fibo_num = 2;

    let num = fibonacci(fibo_num);

    println!("The fibonacci numbher of {} is {}", fibo_num, num);
}


fn convert_temp(n: f64, unit: bool) {
    let c = (n - 32.0) * 5.0 / 9.0;
    let f = n * 9.0 / 5.0 + 32.0;

    if unit {
        println!("{} degrees F is {} degrees C", n, c);
    } else {
        println!("{} degrees C is {} degrees F", n, f);
    }
}


fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}