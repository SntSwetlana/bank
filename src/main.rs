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
fn println_account( account:Account) -> Account{
    println!("{:#?}", account);
    account
}


fn println_accounts( accounts:Vec<Account>) -> Vec<Account>{
    println!("{:#?}", accounts);
    accounts
}

fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Alice"));
    account.balance = 100;
    account = println_account(account);
    account = println_account(account);


    bank.accounts.push(Account::new(1, String::from("Alice")));
    bank.accounts.push(Account::new(4, String::from("Bill")));
    bank.accounts.push(Account::new(5, String::from("Ivy")));

    bank.accounts = println_accounts(bank.accounts);
    bank.accounts = println_accounts(bank.accounts);

    println_bank(bank);
    
}

