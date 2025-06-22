struct User {
    first_name: String,
    last_name: String,
    age: i32,
}

fn main() {
    let user = User {
        first_name: String::from("Aurobindo"),
        last_name: String::from("Patra"),
        age: 32,
    };

    println!("{}", user.first_name);
}
