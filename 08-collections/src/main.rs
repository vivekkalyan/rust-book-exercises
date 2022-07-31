fn main() {
    vectors();
    hashmaps();
}

fn vectors() {
    let v = vec![1,2,3,4,5];

    let third = &v[2];
    println!("The third element is {}", third);


    // let does_not_exist = v[100]; // this will cause the program to panic and crash
    let does_not_exist = v.get(100); // this doesnt crash but returns a Option<T>

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    for i in v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }
}

fn hashmaps() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<String, i32> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    let field_name = String::from("Green");
    let field_value = 20;
    scores.insert(field_name, field_value);

    // println!("field name: {}", field_name); this doesn't work because String is moved
    // println!("field value: {}", field_value); this works because i32 is copied

    let blue_score = scores.get(&String::from("Blue"));
    match blue_score {
        Some(score) => println!("blue score: {}", score),
        None => println!("blue score does not exist"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
