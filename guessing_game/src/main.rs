fn main() {
    println!("Types of strings in Rust");
    println!("-----------------------");
    let name = "Kevin";
    println!("{} is not mutable", name);
    let mut name2 = "kev";
    println!("{} is mutable", name2);
    name2 = "Kev";
    println!("kev was just changed to {}", name2);
    let mut arr:[u32;5] = [1,2,3,4,5];
    for i in 0..arr.len() {
        arr[i] = arr[i] * 2;
    } 
    println!("{:?}", arr);

    let example = "using cargo";

    println!("{example}");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let a = ["Kevin", "Mike", "Robbie", "Maika", "Holly"];

    for element in a {
        println!("the value is: {}", element);
    }

    let my_name = "Kevin";

    for c in my_name.chars() {
        println!("{c}");
    }

    let ex1 = String::from("Hello World");
    println!("{} is my first word",first_word(&ex1));
    let first_w = &ex1[0..5];
    println!("{} is my first word",first_w);


}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}