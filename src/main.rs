use std::io;

fn main() {
    println!("Hello, world!");

    let x: f32 = 10.01;
    println!("{}", x);

    let tup = (1, false, "s");
    println!("{}", tup.2);

    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    arr[3] = 10;
    println!("{:?}, {}", arr, tup.0);

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    println!("input is: {input}");

    let cond = 2 > 3;
    println!("cond is: {cond}");

    println!("test is {}", test(2, 3));

    let mut s = String::from("hello");
    s.push_str("world");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);

    let r3 = &mut s; // BIG PROBLEM

    println!("{}", r3);

    println!("detector => {}", handle_status(Status::Detector));
    println!("exporter => {}", handle_status(Status::Exporter));
    println!(
        "test {:?}",
        last_char_of_first_line("\n\ttest of first line")
    );
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

enum Status {
    Detector,
    Exporter,
}

impl Default for Status {
    fn default() -> Self {
        Self::Detector
    }
}

fn handle_status(status: Status) -> String {
    match status {
        Status::Detector => "detector".to_owned(),
        Status::Exporter => "exporter".to_owned(),
    }
}

fn test(x: i32, y: i32) -> i32 {
    return x + y;
}
