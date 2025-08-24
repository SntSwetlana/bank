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
            balance: 0,
            holder,
        }
    }
}

#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank {     accounts: vec![] }
    }
}
fn println_bank( bank:Bank) -> Bank {
    println!("{:#?}", bank);
    bank
}
fn println_account( account: &Account) {
    println!("{:#?}", account);
}

fn print_num_account(bank: &Bank) {
    println!("Num {:#?}", bank.accounts.len());
}

fn main() {
    let mut bank = Bank::new();
    let account1 = Account::new(1, String::from("Alice"));
    let account2 = Account::new(1, String::from("Who is Alice"));
    let account3 = Account::new(1, String::from("Who is fucking Alice"));
    bank.accounts.push(account1);
    bank.accounts.push(account2);
    bank = println_bank(bank);
    print_num_account(&bank);
    let account_ref = &account3;
    println_account(account_ref);
    println!("{:#?}", account_ref.holder);

}

