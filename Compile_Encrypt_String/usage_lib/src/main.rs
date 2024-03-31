use encrypt_string::encrypt_string;

fn main() {
    let nome = encrypt_string!("I'm encrypted!");
    println!("{}", nome);
}