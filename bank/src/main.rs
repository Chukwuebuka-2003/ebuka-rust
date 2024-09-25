#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String
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
        format!("{} has a balance of {}", self.holder, self.balance)
    }

    fn deposit(&mut self, amount: i32) -> i32 {
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

        Bank { accounts: vec![]}
    }

    fn add_account(&mut self, account: Account) {
        self.accounts.push(account);

    }

    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account| account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts.iter().map(|account | account.summary()).collect::<Vec<String>>()
    }
}



fn main() {

    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("ebuka"));
 
    //let bank = Bank::new();

    account.deposit(5000);
    account.withdraw(3000);

    //let account_ref = &account;

    //print_holder(account.holder);

    //print_accounts(bank.accounts);

    //print_account(account);
    bank.add_account(account);

    println!("{:#?}", bank.summary());
    println!("{}", bank.total_balance());
}

      