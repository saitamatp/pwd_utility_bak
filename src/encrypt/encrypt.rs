use orion::aead as crypto;

pub fn encrypt_string(data: String, password: String) -> Result<Vec<u8>, orion::errors::UnknownCryptoError>{

    let secret_key=crypto::SecretKey::from_slice(password.as_bytes()).expect("Unable to encrypt");
    let ciphertext = crypto::seal(&secret_key, data.as_bytes()).unwrap_or_else(|err|{
       println!("Unable to encrypt due the error-{}",err);
        error()
    });
    Ok(ciphertext)
}

pub fn decryt_string(encrypt: &Vec<u8>, password: String) -> Result<String, orion::errors::UnknownCryptoError>{
    let secret_key=crypto::SecretKey::from_slice(password.as_bytes()).expect("Unable to encrypt");
    let decrypted_data = crypto::open(&secret_key, &encrypt).unwrap_or_else(|err|{
        println!("Unable to decrypt due to incorrect password-Error Details:{}",err);
	error()
    });
    Ok(String::from_utf8(decrypted_data).expect("Unable to convert to string"))
}


pub fn error()->Vec<u8>{
        vec![73,110,118,97,108,105,100,32,80,97,115,115,119,111,114,100]
}
