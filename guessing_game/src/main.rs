fn main() {
    println!("Types of strings in Rust");
    println!("-----------------------");
    let name = "Kevin";
    println!("{} is not mutable", name);
    let mut name2 = "kev";
    println!("{} is mutable", name2);
    name2 = "Kev";
    println!("kev was just changed to {}", name2);
}
