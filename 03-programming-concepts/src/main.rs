fn main() {
    println!("Converting from fahrenheit to celsius");
    let arr_f: [f64; 4] = [-40.0, 32.0, 98.6, 212.0];
    for f in arr_f {
        println!("{}F -> {:.1}C", f, fahrenheit_to_celsius(f));
    }
    println!("");

    println!("Converting from celsius to fahrenheit");
    let arr_c: [f64; 4] = [-40.0, 0.0, 37.0, 100.0];
    for c in arr_c {
        println!("{}C -> {:.1}F", c, celsius_to_fahrenheit(c));
    }
    println!("");

    println!("Generating the nth fibonacci number");
    let arr_fib: [u32; 5] = [0, 1, 2, 12, 20];
    for fib in arr_fib {
        println!("nth fibonacci number: {fib} -> {}", nth_fibonacci(fib))
    }
    println!("");
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn nth_fibonacci(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
    }
}
