use std::collections::HashMap;
fn main() {
    vectors();
    hashmaps();
    statistics();
    convert();
    company();
}

fn vectors() {
    let v = vec![1, 2, 3, 4, 5];

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

fn statistics() {
    let mut v: Vec<i32> = vec![1, 2, 3, 10, 20, 1, 4, 2];
    v.sort();
    let mean = mean(&v);
    println!("mean of {:?}: {}", v, mean);
    let median = median(&v);
    println!("median of {:?}: {}", v, median);
    let mode = mode(&v);
    println!("mode of {:?}: {:?}", v, mode);
}

fn mean(input: &Vec<i32>) -> f32 {
    let mut sum = 0.0;

    for v in input {
        sum += *v as f32;
    }

    return sum / input.len() as f32;
}

fn median(input: &Vec<i32>) -> f32 {
    let input_len = input.len();
    match (input_len % 2) == 0 {
        false => input[input_len / 2] as f32,
        true => (input[input_len / 2 - 1] + input[input_len / 2]) as f32 / 2.0,
    }
}

fn mode(input: &Vec<i32>) -> Vec<i32> {
    let mut counter: HashMap<i32, i32> = HashMap::new();

    for v in input {
        let count = counter.entry(*v).or_insert(0);
        *count += 1;
    }

    let mut max_keys: Vec<i32> = vec![];
    let mut max_count: i32 = -i32::MAX;

    for (item, count) in counter {
        if count > max_count {
            max_keys = vec![item];
            max_count = count;
        } else if count == max_count {
            max_keys.push(item);
        }
    }

    return max_keys;
}

fn convert() {
    let s1: String = String::from("hello");
    let s2: String = String::from("world");
    let s3: String = String::from("amazing");

    println!("{} -> {}", s1, pig_latin(&s1));
    println!("{} -> {}", s2, pig_latin(&s2));
    println!("{} -> {}", s3, pig_latin(&s3));
}

fn pig_latin(s: &String) -> String {
    let first_char = s.chars().nth(0);
    match first_char {
        Some('a') => format!("{}-hay", s),
        Some('e') => format!("{}-hay", s),
        Some('i') => format!("{}-hay", s),
        Some('o') => format!("{}-hay", s),
        Some('u') => format!("{}-hay", s),
        None => String::from(""),
        char => format!("{}-{}ay", &s[1..s.len()], char.unwrap()),
    }
}

fn company() {
    let s1: String = String::from("Add Sally to Engineering");
    let s2: String = String::from("Add Bob to Engineering");
    let s3: String = String::from("Add Amir to Sales");

    let mut company = Company {
        department_members: HashMap::new(),
    };
    company.add_from_string(s1);
    company.add_from_string(s2);
    company.add_from_string(s3);
    println!("Employees: {:?}", &company.get_employees());
    println!(
        "Engineering employees: {:?}",
        &company.get_employees_by_deparment(String::from("Engineering"))
    );
}

#[derive(Debug)]
struct Company {
    department_members: HashMap<String, Vec<String>>,
}

impl Company {
    fn add_from_string(&mut self, s: String) {
        let mut words = s.split_whitespace();
        words.next();
        let employee = String::from(words.next().unwrap());
        words.next();
        let department = String::from(words.next().unwrap());
        let employee_list = self.department_members.entry(department).or_insert(vec![]);
        employee_list.push(employee);
    }

    fn get_employees(&self) -> Vec<String> {
        let mut employees = vec![];

        for (department, employee_list) in &self.department_members {
            for employee in employee_list {
                employees.push(String::from(employee));
            }
        }
        employees.sort();

        employees
    }

    fn get_employees_by_deparment(&self, department: String) -> Vec<String> {
        let mut employees = vec![];

        let employee_list = self.department_members.get(&department).unwrap();
        for employee in employee_list {
            employees.push(String::from(employee));
        }
        employees.sort();

        employees
    }
}
