use firebase_rs::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env::args;
use std::io;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    name: String,
}
#[tokio::main]
async fn main() {
    let user = User {
        name: "chetan baliyan".to_string(),
        age: 30,
        email: "chetan@gmail.com".to_string(),
    };

    println!("Welcome to this app \n\n 1: Register \n 2: Login \n 3: Get user \n 4: Get all users \n 5: delete user");

    let args: Vec<String> = args().collect();
    // println!("{:#?}", commad[1]);

    let mut user_email = String::new();
    let mut user_name = String::new();
    let mut user_age: String = String::new();

    // match command[1].trim().parse() {}

    io::stdin().read_line(&mut user_email).expect("cant read");
    println!("Send me your name");

    io::stdin().read_line(&mut user_name).expect("cant read");
    println!("Send me your age");
    io::stdin().read_line(&mut user_age).expect("cant read");

    let firebase = Firebase::new("https://rust-curd-default-rtdb.firebaseio.com/").unwrap();

    let command: u8 = args[1].trim().parse().expect("error while parsing args");

    match command {
        1 => set_user(&firebase).await,
        _ => println!("Please give valid command"),
    }

    set_user(&firebase).await;

    // let mut user = get_user(&firebase, &response.name).await;
    // println!("{:?}", user);
    // let users = get_users(&firebase).await;

    // println!("{:?}", users);

    // user.email = "update@gmail.com".to_string();

    // let update = update_user(&firebase, &response.name, &user).await;
    // println!("{:?}", update);

    // delete_user(&firebase, &response.name).await;
    // println!("Deleted")
}

async fn set_user(firebase_client: &Firebase) {
    let mut user_email = String::new();
    let mut user_name = String::new();
    let mut user_age: String = String::new();

    io::stdin().read_line(&mut user_email).expect("cant read");
    println!("Send me your name");

    io::stdin().read_line(&mut user_name).expect("cant read");
    println!("Send me your age");
    io::stdin().read_line(&mut user_age).expect("cant read");

    let user_age = user_age.trim().parse().expect("Error while parsing age");

    let user = User {
        name: user_email,
        age: user_age,
        email: user_age,
    };
    let firebase = firebase_client.at("users");

    let _users = firebase.set::<User>(&user).await;

    let created_user = string_to_response(&_users.unwrap().data);

    println!("New user created with email: {:#?}", user.email);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;

    return users.unwrap();
}
async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);

    let user = firebase.get::<User>().await;

    return user.unwrap();
}
async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;
    return string_to_user(&_user.unwrap().data);
}
async fn delete_user(firebase_client: &Firebase, id: &String) {
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}
fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}
