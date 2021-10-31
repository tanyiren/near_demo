use std::collections::HashMap;
use std::convert::TryInto;
/*

*/
// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::{U128};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{
    env, near_bindgen, setup_alloc, AccountId, PanicOnDefault, Promise,
};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Debug, PanicOnDefault)]
pub struct Pool {
    current_reward_total: U128, //当前奖金池总奖金
    current_round: i32,         //当前轮次
    current_num: u8,           //当前开奖号码
    history: Vec<History>,      //获奖历史
    account_list: HashMap<AccountId, AccountInfo>,
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountInfo {
    stake_money: U128, //下注数量
    stake_time: u64,   //下注时间
    stake_round: i32,  //下注轮次
    stake_num: u8,    //下注号码
}

#[derive(BorshDeserialize, BorshSerialize, Debug, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct History {
    account_id: String, //获奖账户
    award_num: U128,    //奖金数量
    round: i32,         //获奖轮次
}

#[near_bindgen]
impl Pool {
    #[init]
    pub fn new() -> Self {
        Self {
            current_reward_total: U128(0),
            current_round: 0,
            current_num: 0,
            history: Vec::new(),
            account_list: HashMap::new(), // 初始化用户列表
        }
    }
    //根据账户获取中奖历史
    pub fn get_award_history_by_account(&self, account: AccountId) {
        for award_mun in self.history.iter() {
            if award_mun.account_id.to_string() == account.to_string() {
                env::log(
                    format!("account {:?}:award history is {:#?}", account, &award_mun).as_bytes(),
                )
            }
        }
    }
    //根据轮次获取中奖历史
    pub fn get_award_history_by_round(&self, round: i32) -> History {
        for award_num in self.history.iter() {
            if award_num.round == round {
                env::log(format!("round {} award history is {:#?}", round, award_num).as_bytes());
                return award_num.clone();
            }
        }
        let history = History {
            account_id: "".to_string(),
            award_num: U128(0),
            round: 0,
        };
        history
    }
    //是否为制定轮次中奖者
    pub fn is_round_winner(&self, account: AccountId, round: i32) -> bool {
        for award_num in self.history.iter() {
            if award_num.round == round {
                if award_num.account_id.to_string() == account.to_string() {
                    true;
                } else {
                    false;
                }
                continue;
            }
        }
        false
    }
    //获取开奖历史
    pub fn get_award_history(&self) {
        // return "happy".to_string();
        env::log(format!("get award history {:?}", self.history).as_bytes());
        env::log(format!("get all pool info {:#?}",self).as_bytes())
    }
    //开奖
    pub fn get_award(&mut self, account: AccountId) {
        assert_eq!(
            env::current_account_id(), // contract owner
            env::signer_account_id(),  // call sign
            "ERR_NOT_ALLOWED"
        );
        //获取奖池总数量
        assert_ne!(
            self.current_reward_total,
            U128(0),
            "current pool reward is 0 !!!"
        );
        assert_ne!(self.account_list.len(), 0, "current participant is 0 !!!");
        let mut reward_lists: Vec<String> = Vec::new();
        for (l, val) in self.account_list.iter() {
            if val.stake_num == self.current_num {
                reward_lists.push(l.to_string()); //将获奖用户放入列表
                                                  //根据用户投入占比分奖励池
                let reward_history = History {
                    account_id: l.to_string().clone(),
                    award_num: U128(
                        val.stake_money.0 / self.current_reward_total.0,
                    ),
                    round: self.current_round,
                };
                Promise::new(account.clone()).transfer(
                    ((val.stake_money.0 + (val.stake_money.0 / self.current_reward_total.0))
                        .try_into()
                        .unwrap_or_default()),
                );
                env::log(
                    format!(
                        "account {} get reward: {}",
                        *l,
                        val.stake_money.0 + (val.stake_money.0 / self.current_reward_total.0)
                    )
                    .as_bytes(),
                );
                self.history.push(reward_history)
            }
        }
    }
    //设置随机数
    pub fn set_award(&mut self, account: AccountId, round: i32) {
        //判断当前账户是否有权限操作该合约
        assert_eq!(env::current_account_id(), account, "ERR_NOT_ALLOWED");
        let seed = env::random_seed();
        let mut arr: [u8; 8] = Default::default();
        arr.copy_from_slice(&seed[..8]);
        self.current_num = arr[0];
        self.current_round = round;
    }
    #[payable]
    pub fn user_bet(&mut self, account_id: AccountId, stake_num: u8) {
        assert!(
            env::attached_deposit() >= 1,
            "deposit near must more than 1!"
        );
        let bet = AccountInfo {
            stake_money: U128(env::attached_deposit()),
            stake_num: stake_num,
            stake_round: self.current_round,
            stake_time: env::block_timestamp(),
        };
        // 用户投注金额
        self.current_reward_total = U128(env::attached_deposit());
        self.account_list.insert(account_id, bet);
    }
}

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use std::collections::HashMap;
    use std::iter::FromIterator;

    fn get_context(predecessor_account_id: AccountId) -> VMContext {
        near_context(predecessor_account_id)
    }

    fn near_context(
        predecessor_account_id: AccountId,
        // epoch_height: EpochHeight,
    ) -> VMContext {
        VMContext {
            current_account_id: "dev-1632968936331-15415893928010".to_string(),
            signer_account_id: "dev-1632968936331-15415893928010".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 1000,
            attached_deposit: 0,
            prepaid_gas: 2 * 10u64.pow(14),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height:0,
        }
    }
    #[test]
    fn test_user_bet(){
        let context = near_context("dev-1632968936331-154158939280A10".to_string());
        testing_env!(context, Default::default(), Default::default());
        let mut contract = Pool::new();
        contract.user_bet("ok3.testnet".to_string(), 11);
    
    }
    #[test]
    fn test_get_award_history(){
        let context =near_context("dev-1632968936331-154158939280A10".to_string()); 
        testing_env!(context, Default::default(), Default::default()); 
        let mut contract = Pool::new();
        contract.get_award_history();
    } 
    #[test]
    fn test_set_award(){
        let context =near_context("dev-1632968936331-154158939280A10".to_string()); 
        testing_env!(context, Default::default(), Default::default()); 
        let mut contract = Pool::new();
        contract.set_award("dev-1632968936331-154158939280A10".to_string(),1);
    } 
}
 