/*
Notes on ownership in Rust
- Purpose of ownership is to limit the ways you can reference and change data
---- Reduces number of bugs and makes code easier to understand
- Rule 1 - Multiple things can refer to a value at the same time but they are all read-only
----- Every value is owned by a single variable at a time (value and binding)
- Rule 2 - A value can only be updated when there are no read-only reference to it
----- Reassigning a value to another variable *moves* the value. References to the old variable can't be used for access anymore
*** Rust avoids unexpected updates

Notes on the borrow system in Rust
- Refs allow us to look at a value without moving it
- Use case -> Make value -> Use value in multiple places, owner changing would be cumbersome. Solution: Use a reference to the value
- You can create many read-only references to a value. These refs can all exist at the same time
- You can't move a value while a ref to the value exists

Thought process with Rust
- When writing a function need to consider whether to use refs or the values themselves
- With data structures need to consider if values are being stored or references to the values

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
            balance: 0,
        }
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
    }

    fn deposit(&mut self, amount: i32) -> i32 {//Struct is being changed, use a mutable reference to self
        self.balance += amount;
        self.balance
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
        self.balance -= amount;
        self.balance
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts
            .iter()
            .map(|account| account.summary())
            .collect::<Vec<String>>()
    }
}

// fn print_account(account: Account) {
//     println!("{:#?}", account);
// } ---- can't print twice

fn print_account(account: &Account) {
    //Context of '&'. The argument needs to be a reference to a value
    println!("{:#?}", account);
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("me"));
    // let mut account = Account::new(1, String::from("me"));
    // let other_bank = bank; <---- moving the bank variable occurs here
    // let list_of_accounts = vec![account]; <--- passing ownership to of the account to the list_of_accounts variable
    println!("{:#?}", bank);
    let account_ref = &account; //Reference to a value
                                // println!("{:#?}", bank);
    // bank.add_account(account); TODO - change this to avoid error with lines below. Use ref?
account.deposit(1234);
    account.withdraw(1233);
    print_account(account_ref);
    print_account(account_ref);
    println!("{:#?}", bank.summary());
    println!("Total balance: {}", bank.total_balance());
    // print_account(account); <--- error - use of moved value account

    //print_holder(account.holder);///moving the holder property out
    //print_account(account);///error because one of the properties have already moved out of the object
}
