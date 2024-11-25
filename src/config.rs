multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::factory::PairTokens;

#[multiversx_sc::module]
pub trait ConfigModule {
    fn is_active(&self) -> bool {
        self.state().get()
    }

    fn check_is_pair_sc(&self, pair_address: &ManagedAddress) {
        require!(
            self.address_pair_map().contains_key(pair_address),
            "Not a pair SC"
        );
    }

    #[view(getState)]
    #[storage_mapper("state")]
    fn state(&self) -> SingleValueMapper<bool>;

    #[storage_mapper("oracle_address")]
    fn oracle_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("fees_collector")]
    fn fees_collector(&self) -> SingleValueMapper<ManagedAddress>;

    #[storage_mapper("jeet_wegld_sc_address")]
    fn jeet_wegld_sc_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[only_owner]
    #[endpoint(setFeesCollector)]
    fn set_fees_collector(&self, fees_collector: ManagedAddress) {
        require!(fees_collector != ManagedAddress::zero(), "Fees collector cannot be zero address");
        self.fees_collector().set(&fees_collector);
    }

    #[storage_mapper("ivl")]
    fn initial_virtual_liquidity(&self) -> SingleValueMapper<BigUint>;

    #[only_owner]
    #[endpoint(setIVL)]
    fn set_initial_virtual_liquidity(&self, virtual_liquidity: BigUint) {
        self.initial_virtual_liquidity().set(&virtual_liquidity);
    }

    #[storage_mapper("issue_token_cost")]
    fn issue_token_cost(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("wegld_unwrap_sc")]
    fn wegld_unwrap_sc(&self) -> SingleValueMapper<ManagedAddress>;
    

    #[view(getTokenSupply)]
    #[storage_mapper("token_supply")]
    fn token_supply(&self) -> SingleValueMapper<BigUint>;

    #[only_owner]
    #[endpoint(setTokenSupply)]
    fn set_token_supply(&self, token_supply: BigUint) {
        require!(token_supply > 0, "Token Supply cannot be zero");
        self.token_supply().set(&token_supply);
    }

    #[view(getNewTokenFee)]
    #[storage_mapper("new_token_fee")]
    fn new_token_fee(&self) -> SingleValueMapper<BigUint>;

    #[only_owner]
    #[endpoint(setNewTokenFee)]
    fn set_new_token_fee(&self, new_token_fee: BigUint) {
        require!(new_token_fee > 0, "Token Fee cannot be zero");
        self.new_token_fee().set(&new_token_fee);
    }

    // #[view(getDexTokenFee)]
    // #[storage_mapper("new_token_fee")]
    // fn dex_token_fee(&self) -> SingleValueMapper<BigUint>;



    // #[only_owner]
    // #[endpoint(setDexTokenFee)]
    // fn set_dex_token_fee(&self, dex_token_fee: BigUint) {
    //     require!(dex_token_fee > 0, "DEX Token Fee cannot be zero");
    //     self.dex_token_fee().set(&dex_token_fee);
    // }

    #[storage_mapper("reach_jeetdex_fee")]
    fn reach_jeetdex_fee(&self) -> SingleValueMapper<BigUint>;

    #[storage_mapper("max_market_cap")]
    fn max_market_cap(&self) -> SingleValueMapper<BigUint>;

    #[only_owner]
    #[endpoint(setMaxMarketCap)]
    fn set_max_market_cap(&self, max_market_cap: BigUint) {
        require!(max_market_cap > 0, "max_market_cap cannot be zero");
        self.max_market_cap().set(&max_market_cap);
    }

    #[view(getBondingTemplateAddress)]
    #[storage_mapper("pair_template_address")]
    fn pair_template_address(&self) -> SingleValueMapper<ManagedAddress>;

    #[only_owner]
    #[endpoint(setBondingTemplateAddress)]
    fn set_Bonding_template_address(&self, address: ManagedAddress) {
        self.pair_template_address().set(&address);
    }


    #[storage_mapper("jeetdex_router_sc_address")]
    fn jeetdex_router_sc_address(&self) -> SingleValueMapper<ManagedAddress>;

    

    #[storage_mapper("pair_map")]
    fn pair_map(&self) -> MapMapper<PairTokens<Self::Api>, ManagedAddress>;

    #[storage_mapper("address_pair_map")]
    fn address_pair_map(&self) -> MapMapper<ManagedAddress, PairTokens<Self::Api>>;

    #[storage_mapper("allowed_token")]
    fn allowed_token(&self) -> SingleValueMapper<TokenIdentifier>;

    #[storage_mapper("jeet_token_id")]
    fn jeet_token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    

}
