# Password Utility (www.pwdutil.com)

## Overview
With so many platforms offering various services which require creation of new Account (username and password) it is becoming difficult to remember so many passwords. I try to solve the problem by storing the passwords in an encrypted file which can then be decrypted to access your passwords. 

## Why use my Service.
Currently I could not find any service which allows you to store all the information with you. I was concerned that all my username and passwords are stored in remote servers which always pose a threat. So, I thought of creating a service that gives you control of your data as the encrypted file is with you.

Link to React [Front end](https://github.com/saitamatp/pwd_utility_front)

## Encryption Algorithm 
I use XChaCha20Poly1305 algorithm with no additional data ([orion::aead crate](https://docs.rs/orion/0.15.5/orion/aead/index.html)).
