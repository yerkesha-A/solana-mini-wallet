struct Wallet {
    balance: f64,
}

impl Wallet {
    fn transfer(&mut self, amount: f64) {
        if !amount.is_finite() || amount <= 0.0 {
            println!("Invalid transfer amount!");
        } else if amount > self.balance {
            println!("Not enough SOL!");
        } else {
            self.balance -= amount;
            println!("Transfer successful!");
            println!("Transferred: {} SOL", amount);
            println!("Remaining balance: {} SOL", self.balance);
        }
    }
}

fn main() {
    let mut wallet = Wallet { balance: 2.0 };

    println!("=== Solana Mini Wallet ===");
    println!("Your balance: {} SOL", wallet.balance);

    let amount: f64 = 0.5;

    println!("Amount to transfer: {} SOL", amount);

    wallet.transfer(amount);
}
