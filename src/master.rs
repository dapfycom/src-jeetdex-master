#![no_std]

multiversx_sc::imports!();
multiversx_sc::derive_imports!();

pub mod config;
pub mod factory;

use bonding::pair_actions::swap::ProxyTrait as _;
use factory::PairTokens;

use bonding::ProxyTrait as _;
use bonding::config::ProxyTrait as _;

use router::config::ProxyTrait as _;

const TOKEN_DECIMALS: usize = 18;


#[multiversx_sc::contract]
pub trait MasterContract:
    config::ConfigModule 
    + factory::FactoryModule
{

    #[init]
    fn init(&self,
        pair_template_address: ManagedAddress,
        token_supply: BigUint,
        fees_collector: ManagedAddress,
        new_token_fee: BigUint,
        // dex_token_fee: BigUint,
        initial_virtual_liquidity: BigUint,
        initial_token_to_buy_with: TokenIdentifier,
        oracle_address: ManagedAddress,
        max_market_cap: BigUint,
        jeetdex_router_sc_address: ManagedAddress,
        issue_token_cost: BigUint,
        wegld_unwrap_sc: ManagedAddress,
        reach_jeetdex_fee: BigUint

    ) {
        self.state().set_if_empty(true);
        require!(pair_template_address != ManagedAddress::zero(), "template cannot be zero");
        require!(token_supply > 0, "Token Supply cannot be zero");
        require!(new_token_fee > 0, "Token Fee cannot be zero");
        require!(initial_virtual_liquidity > 0, "IVL cannot be zero");
        // require!(dex_token_fee > 0, "Dex Token Fee cannot be zero");
        require!(pair_template_address != ManagedAddress::zero(), "oracle cannot be zero");
        require!(max_market_cap > 0, "Max MarketCap cannot be zero");
        require!(issue_token_cost > 0, "issue_token_cost cannot be zero");
        require!(wegld_unwrap_sc != ManagedAddress::zero(), "wegld_unwrap_sc cannot be zero");
        require!(jeetdex_router_sc_address != ManagedAddress::zero(), "jeetdex_router_sc_address cannot be zero");
        require!(reach_jeetdex_fee > 0, "reach_jeetdex_fee cannot be zero");
        
        self.pair_template_address().set_if_empty(pair_template_address);
        self.token_supply().set_if_empty(token_supply);
        self.new_token_fee().set_if_empty(new_token_fee);
        self.fees_collector().set_if_empty(fees_collector);
        self.initial_virtual_liquidity().set_if_empty(initial_virtual_liquidity);
        self.allowed_token().set_if_empty(initial_token_to_buy_with);
        // self.dex_token_fee().set_if_empty(dex_token_fee);
        self.oracle_address().set_if_empty(oracle_address);
        self.max_market_cap().set_if_empty(max_market_cap);
        self.jeetdex_router_sc_address().set_if_empty(jeetdex_router_sc_address);
        self.issue_token_cost().set_if_empty(issue_token_cost);
        self.wegld_unwrap_sc().set_if_empty(wegld_unwrap_sc);
        self.reach_jeetdex_fee().set_if_empty(reach_jeetdex_fee);
    }

    #[endpoint]
    fn upgrade(&self) {
        self.state().set(false);
    }

    #[only_owner]
    #[endpoint]
    fn pause(&self, address: ManagedAddress) {
        if address == self.blockchain().get_sc_address() {
            self.state().set(false);
        } else {
            self.check_is_pair_sc(&address);
            let _: IgnoreValue = self
                .bonding_contract_proxy(address)
                .pause()
                .execute_on_dest_context();
        }
    }

    #[only_owner]
    #[endpoint]
    fn resume(&self, address: ManagedAddress) {
        if address == self.blockchain().get_sc_address() {
            require!(
                self.pair_map().len() == self.address_pair_map().len(),
                "The size of the 2 pair maps is not the same"
            );
            self.state().set(true);
        } else {
            self.check_is_pair_sc(&address);
            let _: IgnoreValue = self
                .bonding_contract_proxy(address)
                .resume()
                .execute_on_dest_context();
        }
    }

    #[only_owner]
    #[endpoint(setJeetDexRouter)]
    fn set_jeetdex_router(&self, address: ManagedAddress, jeet_router_address: ManagedAddress) {
        

        if address == self.blockchain().get_sc_address() {
            self.jeetdex_router_sc_address().set(&jeet_router_address);
        } else {
            self.check_is_pair_sc(&address);
            let _: IgnoreValue = self
                .bonding_contract_proxy(address)
                .set_jeetdex_router(jeet_router_address)
                .execute_on_dest_context();
        }


    }



    #[payable("EGLD")]
    #[endpoint(newToken)]
    fn create_new_token_endpoint(
        &self,
        token_display_name: ManagedBuffer,
        token_ticker: ManagedBuffer,
        db_id: ManagedBuffer,
        token_creator_buy: bool
    ) {
        require!(self.is_active(), "Not active");
        let new_token_fee_cost = self.call_value().egld_value().clone_value();

        if !self.new_token_fee().is_empty(){
            require!(
                new_token_fee_cost == self.new_token_fee().get(),
                "New Token Fee is not correct"
            );
            //Send fees to fees collector
            // self.send().direct_egld(&self.fees_collector().get(),&new_pair_fee_cost);

        }

        let caller = self.blockchain().get_caller();

        require!(self.token_supply().get() > 0, "Token Supply cannot be zero");

        if self.blockchain().get_gas_left() > 150000000 {
            self.send()
                .esdt_system_sc_proxy()
                .issue_fungible(
                    self.issue_token_cost().get(),
                    &token_display_name,
                    &token_ticker,
                    &BigUint::from(self.token_supply().get()),
                    FungibleTokenProperties {
                        num_decimals: TOKEN_DECIMALS,
                        can_freeze: false,
                        can_wipe: false,
                        can_pause: false,
                        can_mint: false,
                        can_burn: false,
                        can_change_owner: false,
                        can_upgrade: false,
                        can_add_special_roles: false,
                    },
                )
                .with_gas_limit(150000000)
                .async_call()
                .with_callback(
                    self.callbacks()
                        .token_issue_callback(&caller,&db_id,&token_creator_buy),
                )
                .call_and_exit();
        }else{
            sc_panic!("Not enough gas left {}",self.blockchain().get_gas_left());
        }

    }

    #[only_owner]
    #[allow_multiple_var_args]
    #[endpoint(upgradeToken)]
    fn upgrade_pair_endpoint(
        &self,
        first_token_id: TokenIdentifier,
        second_token_id: TokenIdentifier,
    ) {
        require!(self.is_active(), "Not active");

        require!(first_token_id != second_token_id, "Identical tokens");
        require!(
            first_token_id.is_valid_esdt_identifier(),
            "First Token ID is not a valid esdt token ID"
        );
        require!(
            second_token_id.is_valid_esdt_identifier(),
            "Second Token ID is not a valid esdt token ID"
        );

        require!(
            self.allowed_token().get() == second_token_id,
            "Second Token ID is not allowed"
        );


        let bonding_address = self.get_pair(first_token_id.clone(), second_token_id.clone());
        require!(!bonding_address.is_zero(), "Bonding does not exists");


        self.upgrade_bonding(
            bonding_address
        );
    }


    #[callback]
    fn token_issue_callback(
        &self,
        caller: &ManagedAddress,
        db_id: &ManagedBuffer,
        token_creator_buy: &bool,
        #[call_result] result: ManagedAsyncCallResult<()>,
    ) {
        let (token_id, returned_tokens) = self.call_value().egld_or_single_fungible_esdt();
        match result {
            ManagedAsyncCallResult::Ok(()) => {
                let bonding_address = self.create_bonding(db_id.clone());
                
                let _: IgnoreValue = self.jeetdex_router_proxy(self.jeetdex_router_sc_address().get())
                .set_temp_degen_pair(bonding_address.clone())
                .execute_on_dest_context();
                
                self.pair_map().insert(
                    PairTokens {
                        first_token_id: token_id.clone().unwrap_esdt(),
                        second_token_id: self.allowed_token().get(),
                    },
                    bonding_address.clone(),
                );
                self.address_pair_map().insert(
                    bonding_address.clone(),
                    PairTokens {
                        first_token_id: token_id.clone().unwrap_esdt(),
                        second_token_id: self.allowed_token().get(),
                    },
                );
            
                let _: IgnoreValue = self
                    .bonding_contract_proxy(bonding_address.clone())
                    .set_token_identifier(token_creator_buy,caller)
                    // .with_multi_token_transfer(payments)
                    .with_esdt_transfer(EsdtTokenPayment::new(token_id.clone().unwrap_esdt(), 0, returned_tokens))
                    .execute_on_dest_context();
                

                let remaining_fee = self.new_token_fee().get() - self.issue_token_cost().get();
                if remaining_fee > 0{
                    self.send().direct_egld(&self.fees_collector().get(), &remaining_fee);
                }


            }
            ManagedAsyncCallResult::Err(_) => {
                if token_id.is_egld() && returned_tokens > 0u64 {
                    self.send().direct_egld(caller, &self.new_token_fee().get());
                }
            }
        }
    }

  
    #[proxy]
    fn oracle_proxy(&self, to: ManagedAddress) -> oracle_proxy::Proxy<Self::Api>;
    #[proxy]
    fn jeetdex_router_proxy(&self, to: ManagedAddress) -> router::Proxy<Self::Api>;
    #[proxy]
    fn bonding_contract_proxy(&self, to: ManagedAddress) -> bonding::Proxy<Self::Api>;

}

mod oracle_proxy {
    multiversx_sc::imports!();

    #[multiversx_sc::proxy]
   pub trait xExchangeOracleContract {
        #[view(getAmountOut)]
        fn get_amount_out_view(&self, token_in: TokenIdentifier, amount_in: BigUint);

        #[payable("*")]
        #[endpoint(swapTokensFixedInput)]
        fn swap_tokens_fixed_input(&self,token_out: TokenIdentifier,amount_out_min: BigUint);

        #[payable("*")]
        #[endpoint(unwrapEgld)]
        fn unwrap_egld(&self);

        #[payable("EGLD")]
        #[endpoint(wrapEgld)]
        fn wrap_egld(&self) -> EsdtTokenPayment;
    }
}