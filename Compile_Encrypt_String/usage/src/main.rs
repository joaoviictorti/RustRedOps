use obf::obf;

fn main() {
    let name = obf!("I'm encrypted!");
    println!("{}", name);
}