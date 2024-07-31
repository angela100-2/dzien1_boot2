use candid::Principal;
use ic_cdk::caller;
use std::{cell::RefCell, collections::HashMap};
use user::UserData;

pub mod user;

thread_local! {
    static CHAT: RefCell<HashMap<[Principal; 2], Vec<String>>> = RefCell::default();
    static USERS: RefCell<HashMap<Principal, UserData>> = RefCell::default();
}

#[ic_cdk::update]
fn register(nick: String) {
    let user = caller();
    if user == Principal::anonymous() {
        panic!("Who is this??")
    }

    USERS.with_borrow_mut(|users| users.insert(user, UserData::new(nick)));
}

#[ic_cdk::query]
fn get_users() -> HashMap<Principal, UserData>{
    USERS.with_borrow(|users| {
        users.clone()
    })
}

#[ic_cdk::query]
fn get_user(user: Principal) -> Option<UserData>{
    USERS.with_borrow(|users| {
        users.get(&user).cloned()
    })
}

#[ic_cdk::query]
fn get_chat(chat_path: [Principal; 2]) -> Option<Vec<String>> {
    CHAT.with_borrow(|chat| chat.get(&chat_path).cloned())
}

#[ic_cdk::update]
fn add_chat_msg(msg: String, user2: Principal) {
    let user1 = caller();
    if user1 == Principal::anonymous() {
        panic!("Who is this??")
    }

    let is_user_registered = USERS.with_borrow(|users| {
        users.contains_key(&user1)
    });
    if !is_user_registered {
        panic!("You no register. Go register.")
    }

    let mut principals: [Principal; 2] = [user1, user2];
    principals.sort();

    CHAT.with_borrow_mut(|chat| {
        let mut_chat = chat.get_mut(&principals);

        if let Some(chat_msgs) = mut_chat {
            chat_msgs.push(msg)
        } else {
            chat.insert(principals, vec![msg]);
        }
    })
}