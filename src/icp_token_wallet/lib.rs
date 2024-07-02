mod token;
use token::Token;
use ic_cdk_macros::*;

thread_local! {
    static TOKEN: Token = Token::new("IRCRC2".to_string(), 1_000_000);
}

#[update]
async fn transfer(from: String, to: String, amount: u64) -> Result<(), String> {
    TOKEN.with(|token| token.transfer(from, to, amount))
}

#[query]
async fn balance_of(account: String) -> u64 {
    TOKEN.with(|token| token.balance_of(account))
}

#[update]
async fn send_tokens(to: String, amount: u64) -> Result<(), String> {
    let caller = ic_cdk::caller().to_string();
    transfer(caller, to, amount).await
}

#[query]
async fn get_balance() -> u64 {
    let caller = ic_cdk::caller().to_string();
    balance_of(caller).await
}
