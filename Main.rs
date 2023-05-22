use std::io;

struct BankAccount {
    identification: u64,
    balance: u64,
}

impl BankAccount {
    //fn addAccount(&self, id: u32, amount: u32) {
        //self.id = id;
        //self.balance = amount;
    //}
    fn set_id(&mut self, id: u64){
        self.identification = id;
    }

    fn set_balance(&mut self, amount: u64){
        self.balance = amount;
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }
    
    fn withdraw(&mut self, amount: u64) {
        self.balance -= amount;
    }
}


fn main() {
 let mut finished: bool = false;
 let mut account_created = false;
 let mut input = String::new();
 
 while finished != true {
    println!("Select an option: ");
    println!("Option 1: create your account.");
    println!("Option 2: deposit to your account.");
    println!("Option 3: withdraw from your account.");
    println!("Option 4: quit the program.");
    input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line.");
    let option: u64 = input.trim().parse().expect("Input is not an integer.");
    input.clear();
    let mut acct1 = BankAccount {
        identification: 0,
        balance: 0,
    };
    
    if option == 1 {
        //get an id number from the user
        println!("Please provide an id number.");
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        let id: u32 = input.trim().parse().expect("Input is not an Integer");
        input.clear();
        
        //get an initial balance from the user
        println!("Please provide an intitial balance for the account.");
        input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line.");
        let amount: u64 = input.trim().parse().expect("Input is not an integer.");
        input.clear();

        //create an instance of bank account
        acct1.set_id(id);
        acct1.set_balance(amount);

        println!("Your account has been successfully created, you can now deposit and withdraw.");
        account_created = true;

    }
    
    //deposit option
    else if option == 2 {
        if account_created == true {
            println!("How much do you want to deposit?: ");
            input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            let deposit_amount: u64 = input.trim().parse().expect("Input is not an integer.");
            input.clear();
            acct1.deposit(deposit_amount);
            println!("you have deposited ${}", deposit_amount);
        }
        else {
            println!("you must create an account first!");
        }
    }
    
    //withdraw option
    else if option == 3 {
        if account_created == true {
            println!("How much do you want to withdraw?: ");
            input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line.");
            input.clear();
            let withdraw_amount: u64 = input.trim().parse().expect("Input is not an integer.");
            acct1.withdraw(withdraw_amount);
            println!("you have withdrawn ${}", withdraw_amount);
        }
        else {
            println!("you must create an account first!");
        }
    }
    
    //quit option
    else if option == 4 {
        println!("Good bye!");
        finished = true;
    }
    
    //error option
    else {
        println!("That is not a valid option!");
    }
 }
}


//main!();