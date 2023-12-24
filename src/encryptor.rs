use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
use text_colorizer::Colorize;

pub fn encrypt(data: &str, encryption_type: &str) -> String {
    match encryption_type {
        "AES" => {
            aes_encrypt(&data)
        },
        _ => {
            panic!("{} no encryption type: {}. available encryption types: 'AES'", "error:".red().bold(), encryption_type);
        }
    }
}

fn aes_encrypt(data: &str) -> String {
    let key = GenericArray::from([0u8; 16]);
    let cipher = Aes128::new(&key);

    let data_bytes = data.as_bytes();

    let mut encrypted_data = Vec::new();

    for chunk in data_bytes.chunks(16) {
        let mut block = GenericArray::clone_from_slice(&[0u8; 16]);
        for (i, &b) in chunk.iter().enumerate() {
            block[i] = b;
        }

        cipher.encrypt_block(&mut block);

        encrypted_data.extend(block.iter().map(|b| format!("{:02x}", b)));
    }

    encrypted_data.join("")
}
