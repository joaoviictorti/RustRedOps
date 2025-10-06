use proc_macro::TokenStream;
use quote::quote;
use rand::{thread_rng, Rng};
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn obf(input: TokenStream) -> TokenStream {
    let input_str = parse_macro_input!(input as LitStr);
    let mut rand = thread_rng();
    let key: u8 = rand.gen();
    let encrypted_str = simple_encrypt(&input_str.value(), key);

    // Generates the Rust code that decrypts the string at runtime.
    let gen = quote! {
        {
            // Built-in function to decrypt the string
            fn simple_decrypt(input: &str, key: u8) -> String {
                let input = input.to_string();
                let string_bytes = input.as_bytes();
                let result: Vec<u8> = string_bytes.iter().map(|valor| valor ^ key).collect();
                let decrypt = String::from_utf8_lossy(&result);

                decrypt.to_string()
            }

            // The encrypted string is decrypted at runtime
            simple_decrypt(#encrypted_str, #key)
        }
    };

    gen.into()
}

// Simplified implementation of cryptography
fn simple_encrypt(input: &str, key: u8) -> String {
    let string = input.to_string();
    let string_bytes = string.as_bytes();
    let result: Vec<u8> = string_bytes.iter().map(|valor| valor ^ key).collect();
    let encrypt = String::from_utf8_lossy(&result);

    encrypt.to_string()
}
