use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite color");
    let field_value = 50;

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{:?}", map);
    println!("{:?}", field_name);
    println!("{:?}", field_value);
}
