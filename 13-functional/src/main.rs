fn main() {
    closures();
    iterators();
}

fn closures() {
    #[derive(Debug, PartialEq, Copy, Clone)]
    enum ShirtColor {
        Red,
        Blue,
    }

    struct Inventory {
        shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let mut num_red = 0;
            let mut num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red += 1,
                    ShirtColor::Blue => num_blue += 1,
                }
            }
            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // immutable borrow
    let list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list[1]);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    // mutable borrow
    let mut list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // at this point the closure has done a mutable borrow of the variable list and thus list
    // cannot be immutably borrowed for println
    // println!("Before calling closure: {:?}", list); -- this will cause an error
    borrows_mutably();
    println!("After calling closure: {:?}", list);

    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

fn iterators() {
    let v = vec![1, 2, 3];
    let v_iter1 = v.iter();
    for val in v_iter1 {
        println!("Got: {}", val);
    }

    let mut v_iter2 = v.iter();
    assert_eq!(v_iter2.next(), Some(&1));
    assert_eq!(v_iter2.next(), Some(&2));
    assert_eq!(v_iter2.next(), Some(&3));
    assert_eq!(v_iter2.next(), None);

    // consuming adaptors
    let v_iter3 = v.iter();
    let total: i32 = v_iter3.sum();
    assert_eq!(total, 6);

    // iterator adaptors
    let v2: Vec<_> = v.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // capture environment
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe {
                size: 10,
                style: String::from("sneaker")
            },
            Shoe {
                size: 10,
                style: String::from("boot")
            },
        ]
    );
}
