/*
Notes on ownership in Rust
- Purpose of ownership is to limit the ways you can reference and change data
---- Reduces number of bugs and makes code easier to understand
- Rule 1 - Multiple things can refer to a value at the same time but they are all read-only
----- Every value is owned by a single variable at a time (value and binding)
- Rule 2 - A value can only be updated when there are no read-only reference to it
----- Reassigning a value to another variable *moves* the value. References to the old variable can't be used for access anymore
*** Rust avoids unexpected updates


 */

#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance:0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts:Vec<Account>
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

// fn print_account(account: Account) {
//     println!("{:#?}", account);
// } ---- can't print twice


fn print_account(account: Account) -> Account {
    println!("{:#?}", account);
    account //implicit return
}

fn print_holder(holder: String) {
    println!("{:#?}", holder)
}

fn main() {
    let bank = Bank::new();
    // let account = Account::new(1, String::from("me"));
    let mut account = Account::new(1, String::from("me"));
    // let other_bank = bank; <---- moving the bank variable occurs here
    // let list_of_accounts = vec![account]; <--- passing ownership to of the account to the list_of_accounts variable
    println!("{:#?}", bank);
    // println!("{:#?}", bank);
    account = print_account(account);
    account = print_account(account);
    // print_account(account); <--- error - use of moved value account

    //print_holder(account.holder);///moving the holder property out
    //print_account(account);///error because one of the properties have already moved out of the object
}
