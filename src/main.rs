use std::env;
use std::str::FromStr;

// use web3::contract::{Contract, Options};
use web3::types::{H160, U256, Address, TransactionParameters};
use web3::ethabi::ethereum_types;

use secp256k1::SecretKey;

fn wei_to_eth(wei_val: U256) -> f64{
    let res = wei_val.as_u128() as f64;
    res/1_000_000_000_000_000_000.0
}

#[tokio::main]
async fn main() -> web3::Result<()>{
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("INFURA_RINKEBY").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);
    let mut accounts = web3s.eth().accounts().await?;

    accounts.push(H160::from_str(&env::var("ACCOUNT_ADDRESS").unwrap()).unwrap());
    println!("Accoutns: {:?}", accounts);

    for account in accounts{
        let balance = web3s.eth().balance(account, None).await?;
        println!("Balance in ETH in account no {:?} is {}", account, wei_to_eth(balance));
    }

	let prvk = SecretKey::from_str(&env::var("PROJECT_SECRET").unwrap());

	for account in accounts{
		let tx_object = TransactionParameters{
			to: Some(account),
			value: U256::exp10(17),
			..Default::default()
		};
	}
	

	let signed = web3s.accounts().sign_transaction(tx_object, &prvk).await?;
	let result = web3s.eth().send_raw_transaction(signed.raw_transaction).await?;
	
	println!("Tx succeeded with hash: {}", result);
    
	Ok(())
}

