contract;

use std::{
    auth::msg_sender,
    context::msg_amount,
    token::*,
    storage::StorageMap,
    blockchain::timestamp,
};

storage {
    // Total staked amount
    total_staked: u64,
    // Mapping of user address to staked amount
    staked_amounts: StorageMap<Address, u64>,
    // Mapping of user address to timestamp of stake
    stake_timestamps: StorageMap<Address, u64>,
    // Liquid staking token contract
    ls_token: ContractId,
}

abi LiquidStaking {
    #[storage(read, write)]
    fn stake();
    
    #[storage(read, write)]
    fn unstake(amount: u64);
    
    #[storage(read)]
    fn get_staked_amount(user: Address) -> u64;
    
    #[storage(read)]
    fn get_total_staked() -> u64;
}

impl LiquidStaking for Contract {
    // Stake ETH and receive liquid staking tokens
    #[storage(read, write)]
    fn stake() {
        // Get the sender's address and staked amount
        let sender = msg_sender().unwrap();
        let amount = msg_amount();
        
        require(amount > 0, "Cannot stake 0 amount");
        
        // Update total staked amount
        storage.total_staked += amount;
        
        // Update user's staked amount
        let current_stake = storage.staked_amounts.get(sender);
        storage.staked_amounts.insert(sender, current_stake + amount);
        
        // Record stake timestamp
        storage.stake_timestamps.insert(sender, timestamp());
        
        // Mint liquid staking tokens to the sender
        // Assuming 1:1 ratio for simplicity
        let ls_token_contract = abi(LSToken, storage.ls_token.read());
        ls_token_contract.mint(sender, amount);
    }
    
    #[storage(read, write)]
    fn unstake(amount: u64) {
        let sender = msg_sender().unwrap();
        let staked_amount = storage.staked_amounts.get(sender);
        
        require(amount > 0, "Cannot unstake 0 amount");
        require(amount <= staked_amount, "Insufficient staked balance");
        
        // Ensure minimum staking period has passed
        let stake_time = storage.stake_timestamps.get(sender);
        require(
            timestamp() >= stake_time + 86400, // 24 hours in seconds
            "Minimum staking period not met"
        );
        
        // Update total staked amount
        storage.total_staked -= amount;
        
        // Update user's staked amount
        storage.staked_amounts.insert(sender, staked_amount - amount);
        
        // Burn liquid staking tokens
        let ls_token_contract = abi(LSToken, storage.ls_token.read());
        ls_token_contract.burn(sender, amount);
        
        // Transfer ETH back to user
        // Note: In a real implementation, you'd need to handle rewards
        transfer(amount, sender);
    }
    
    #[storage(read)]
    fn get_staked_amount(user: Address) -> u64 {
        storage.staked_amounts.get(user)
    }
    
    #[storage(read)]
    fn get_total_staked() -> u64 {
        storage.total_staked
    }
}

abi LSToken {
    fn mint(to: Address, amount: u64);
    fn burn(from: Address, amount: u64);
}