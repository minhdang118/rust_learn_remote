fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 5 {
                break;
            }
            if count == 3 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}\n");

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!\n");

    let x = fib(30);

    println!("Fibonacci number: {x}");
}

fn fib(x: i32) -> i32 {
    if (x == 0) || (x == 1) {
        return x;
    } else {
        return fib(x-1) + fib(x-2);
    }
}
