use chrono::Utc;
use rusoto_core::Region;
use rusoto_s3::S3Client;
use std::env;

mod database;
mod s3;
mod passwordhash;
mod account;

//use database::{create_tables, insert_user, insert_subscription, insert_file_to_database, insert_file_s3_and_database, get_user_id_by_email, get_files_by_user_id, is_subscribed, get_user_balance, is_broke, verify_password_by_email};
use database::{create_tables, insert_user, insert_subscription, get_user_id_by_username, get_user_balance, update_user_balance};
use s3::{create_s3_client, insert_file_to_s3};
use passwordhash::{hash_password, verify_password};
use account::User;
use dotenv::{dotenv};

use crate::database::{is_subscribed, verify_password_by_email, is_broke};


fn main() {
    // Load environment variables from the .env file
    let conn = rusqlite::Connection::open_in_memory().expect("Failed to open in-memory database");
    create_tables(&conn).expect("Failed to create tables");

    let user_id = insert_user(&conn, "test@example.com", "testuser", &hash_password("testpassword")).expect("Failed to insert user");
    println!("{}", user_id);

   let balance = get_user_balance(&conn, user_id);

   match balance {
      Ok(b) => println!("User balance: {}", b),
      Err(err) => println!("Error getting user balance: {:?}", err),
  }

  let broke1 = is_broke(&conn, user_id, 10.0);

  match broke1 {
   Ok(true) => println!("GOOD BOYS"),
   Ok(false) => println!("bad boys"),
   Err(err) => println!("error: {:?}", err),
}

  let _jojo = update_user_balance(&conn, user_id, 10.0);

  let broke2 = is_broke(&conn, user_id, 10.0);

  match broke2 {
   Ok(true) => println!("bad boys"),
   Ok(false) => println!("good boys"),
   Err(err) => println!("error: {:?}", err),
}

  let balance = get_user_balance(&conn, user_id);

   match balance {
      Ok(b) => println!("User balance: {} MONKEY", b),
      Err(err) => println!("Error getting user balance: {:?}", err),
  }




   /*
   let user_id2 = insert_user(&conn, "test2@example.com", "testusersec", &hash_password("testpassword")).expect("Failed to insert user");
   println!("{}", user_id2);

    let user_id_retrieved = get_user_id_by_username(&conn, "testuser");

   match user_id_retrieved {
         Ok(Some(user_id)) => {
         println!("User ID retrieved: {}", user_id);
      }
      Ok(None) => {
         println!("User not found");
      }
      Err(err) => {
         println!("Error retrieving user ID: {:?}", err);
      }
   }

    let start_date = Utc::now();
    let subscription_id = insert_subscription(&conn, user_id, user_id2, "plan_a", start_date, 5.0).expect("Failed to insert subscription");
    println!("{}", subscription_id);

    
    let login_attempt = verify_password_by_email(&conn, "test@example.com", "testpassword");
    match login_attempt {
       Ok(is_subscribed) => {
           if is_subscribed {
               println!("login success");
           } else {
               println!("login failed");
           }
       }
       Err(err) => {
           match err {
               rusqlite::Error::QueryReturnedNoRows => {
                   println!("User not found");
               }
               _ => {
                   println!("Error checking subscription: {:?}", err);
               }
           }
       }
   }
    
    
    */
    
   /*
   let is_subscribed_result = is_subscribed(&conn, user_id, user_id2);
   match is_subscribed_result {
      Ok(is_subscribed) => {
         if is_subscribed {
               println!("is subbed");
         } else {
               println!("not subbed");
         }
      }
      Err(err) => {
         println!("Error checking subscription: {:?}", err);
      }
   }

    let login_attempt = verify_password_by_email(&conn, "test@example.com", "testpassword");
   match login_attempt {
      Ok(is_subscribed) => {
          if is_subscribed {
              println!("login success");
          } else {
              println!("login failed");
          }
      }
      Err(err) => {
          match err {
              rusqlite::Error::QueryReturnedNoRows => {
                  println!("User not found");
              }
              _ => {
                  println!("Error checking subscription: {:?}", err);
              }
          }
      }
  }
   
    */
    


   /*
    // Initialize the connection to the database
    let conn = rusqlite::Connection::open_in_memory().expect("Failed to open in-memory database");
    create_tables(&conn).expect("Failed to create tables");

    // Initialize S3 client
    let s3_client = create_s3_client();

    // Set up environment variables for AWS credentials
    env::set_var("AWS_ACCESS_KEY_ID", "your_access_key_id");
    env::set_var("AWS_SECRET_ACCESS_KEY", "your_secret_access_key");

    // Test user insertion
    let user_id = insert_user(&conn, "test@example.com", "testuser", &hash_password("testpassword")).expect("Failed to insert user");
    
    // Test subscription insertion
    let start_date = Utc::now();
    let subscription_id = insert_subscription(&conn, user_id, 0, "plan_a", start_date).expect("Failed to insert subscription");

    // Test file insertion to the database
    let file_id = insert_file_to_database(&conn, user_id, "test_file.txt", "File description", "uploads/test_file.txt", start_date).expect("Failed to insert file");

    // Test file insertion to S3 and the database
    let vector_of_ones: Vec<u8> = vec![1; 10];
    insert_file_s3_and_database(&conn, user_id, "test_file.txt", "File description", &s3_client, vector_of_ones, "rustproejctbucket").expect("Failed to insert file to S3");

    // Test getting user ID by email
    let retrieved_user_id = get_user_id_by_email(&conn, "test@example.com").expect("Failed to get user ID by email").unwrap();
    assert_eq!(user_id, retrieved_user_id);

    // Test getting files by user ID
    let files = get_files_by_user_id(&conn, user_id).expect("Failed to get files by user ID");
    assert_eq!(files.len(), 2); // Assuming two files are inserted

    // Test subscription status
    let is_subscribed = is_subscribed(&conn, user_id, 0).expect("Failed to check subscription status");
    assert!(is_subscribed);

    // Test getting user balance
    let balance = get_user_balance(&conn, user_id).expect("Failed to get user balance");
    assert_eq!(balance, 0.0); // Assuming the default balance is 0.0

    // Test if the user is broke
    let is_broke = is_broke(&conn, user_id, 10).expect("Failed to check if the user is broke");
    assert!(is_broke); // Assuming the balance is less than 10

    // Test password verification
    let is_password_correct = verify_password_by_email(&conn, "test@example.com", "testpassword").expect("Failed to verify password");
    assert!(is_password_correct);

    // Clean up: You may want to drop tables or close the connection after testing.
   
   
    */

}