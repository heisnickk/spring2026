mod bank_account;

use bank_account::BankAccount;

fn main() {
    let mut account = BankAccount::new(200.0);

    println!("Initial balance: {}", account.balance());

    account.deposit(90.0);
    println!("After deposit: {}", account.balance());

    account.withdraw(50.0);
    println!("After withdrawal: {}", account.balance());
}