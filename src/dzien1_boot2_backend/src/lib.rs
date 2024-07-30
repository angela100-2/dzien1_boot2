use std::{cell::RefCell, collections::HashMap};
use candid::Principal;
use ic_cdk::caller;

thread_local! {
    static CHAT: RefCell<HashMap<[Principal; 2], Vec<String>>> = RefCell::default();
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