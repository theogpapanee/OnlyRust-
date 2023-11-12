OnlyRust💗

Shreyes Bharat - sbharat2

Christopher Nee - cnee2
# Project Introduction:
We will be building a subscription-based media-sharing platform that allows files to be stored on the cloud, with other users being able to access them. We will be implementing an account system with password protection. We will also be implementing a currency system, by which users can pay to access certain files. This will be the outline of an online marketplace. 

# Technical Overview:
1. First create a HTTP website, perhaps using tide.

2. Create an account system, protected by user-generated passwords

3. Passwords are encrypted and stored somehow…

4. Implement a cloud-based file storage system for users to upload and share media.

5. Implement a subscription system wherein users may pay to access the files of another.

## Project Structure:
**root**

|Src

|--main.rs      # Main application entry point

|--models       # Database models

|--routes       # Actix web routes

|--password.rs  # Password hashing and verification

|--...           # Other files

| Cargo.toml       # Rust project configuration

## Dependencies:
actix-web = "4" //for webdev

tokio = { version = "1", features = ["full"] } //for async tasks

sqlx = { version = "0.5", features = ["postgres"] } // for database 

argon2 = "0.9" //for encryption

serde = { version = "1", features = ["derive"] } //serialization

# Possible Challenges:
* Limited rust webdev

* External Service/API integration

* Database integration

* Security of user data/files
# References:

We are basing our project off of online marketplaces, such as OnlyFans, ITunes, Patreon.
