fn main() {
    generics();
}

fn generics() {
    fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![34, 50, 10, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 't', 'b', 'y', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    struct Point<T, U> {
        x: T,
        y: U,
    }

    let _both_integer = Point { x: 5, y: 10 };
    let _both_float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };
}
