multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::config;
use bonding::ProxyTrait as _;
use bonding::config::ProxyTrait as _;
use bonding::contexts::base::State;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, TypeAbi)]
pub struct PairTokens<M: ManagedTypeApi> {
    pub first_token_id: TokenIdentifier<M>,
    pub second_token_id: TokenIdentifier<M>,
}

#[derive(ManagedVecItem, TopEncode, TopDecode, PartialEq, TypeAbi)]
pub struct PairContractMetadata<M: ManagedTypeApi> {
    first_token_id: TokenIdentifier<M>,
    second_token_id: TokenIdentifier<M>,
    address: ManagedAddress<M>,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone,ManagedVecItem)]
pub struct PairData<M: ManagedTypeApi> {
    pub first_token_id: TokenIdentifier<M>,
    pub second_token_id: TokenIdentifier<M>,
    pub first_token_reserve: BigUint<M>,
    pub second_token_reserve: BigUint<M>,
    pub owner_fee_percent: u64,
    pub market_cap: BigUint<M>,
    pub db_id: ManagedBuffer<M>,
    pub state: State
    
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi, Clone,ManagedVecItem)]
pub struct PairContractData<M: ManagedTypeApi> {
    pub sc_address: ManagedAddress<M>,
    pub first_token_id: TokenIdentifier<M>,
    pub second_token_id: TokenIdentifier<M>,
    pub first_token_reserve: BigUint<M>,
    pub second_token_reserve: BigUint<M>,
    pub owner_fee_percent: u64,
    pub market_cap: BigUint<M>,
    pub db_id: ManagedBuffer<M>,
    pub state: State
}


#[multiversx_sc::module]
pub trait FactoryModule: config::ConfigModule {
    #[proxy]
    fn bonding_deploy_proxy(&self) -> bonding::Proxy<Self::Api>;
    #[proxy]
    fn bonding_view_proxy(&self, to: ManagedAddress) -> bonding::Proxy<Self::Api>;



    fn create_bonding(
        &self,
        db_id: ManagedBuffer
    ) -> ManagedAddress {
        require!(
            !self.pair_template_address().is_empty(),
            "pair contract template is empty"
        );

        let (new_address, ()) = self
            .bonding_deploy_proxy()
            .init(
                self.allowed_token().get(),
                self.fees_collector().get(),
                self.initial_virtual_liquidity().get(),
                // self.dex_token_fee().get(),
                self.oracle_address().get(),
                self.max_market_cap().get(),
                self.jeetdex_router_sc_address().get(),
                self.issue_token_cost().get(),
                self.wegld_unwrap_sc().get(),
                self.reach_jeetdex_fee().get(),
                db_id
            )
            .deploy_from_source(
                &self.pair_template_address().get(),
                CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
            );

        new_address
    }

    fn upgrade_bonding(
        &self,
        bonding_address: ManagedAddress
    ) {

        self.bonding_deploy_proxy()
            .contract(bonding_address)
            .init(
                self.allowed_token().get(),
                self.fees_collector().get(),
                self.initial_virtual_liquidity().get(),
                // self.dex_token_fee().get(),
                self.oracle_address().get(),
                self.max_market_cap().get(),
                self.jeetdex_router_sc_address().get(),
                self.issue_token_cost().get(),
                self.wegld_unwrap_sc().get(),
                self.reach_jeetdex_fee().get(),
                ManagedBuffer::new()
            )
            .upgrade_from_source(
                &self.pair_template_address().get(),
                CodeMetadata::UPGRADEABLE | CodeMetadata::READABLE | CodeMetadata::PAYABLE_BY_SC,
            );
    }




    #[view(getAllBondingMetadata)]
    fn get_all_pair_contract_metadata(&self) -> MultiValueEncoded<PairContractMetadata<Self::Api>> {
        let mut result = MultiValueEncoded::new();
        for (k, v) in self.pair_map().iter() {
            let pair_metadata = PairContractMetadata {
                first_token_id: k.first_token_id,
                second_token_id: k.second_token_id,
                address: v,
            };
            result.push(pair_metadata);
        }
        result
    }

    #[view(getAllBondingData)]
    fn get_all_pair_contract_data(&self) -> MultiValueEncoded<PairContractData<Self::Api>> {
        let mut result = MultiValueEncoded::new();
        for (k, v) in self.pair_map().iter() {
            
            let pair_data: PairData<Self::Api> = self.bonding_view_proxy(v.clone()).get_pair_data().execute_on_dest_context_readonly();

            let pair_contract_data = PairContractData{
                sc_address: v,
                first_token_id: pair_data.first_token_id,
                second_token_id: pair_data.second_token_id,
                first_token_reserve: pair_data.first_token_reserve,
                second_token_reserve: pair_data.second_token_reserve,
                owner_fee_percent: pair_data.owner_fee_percent,
                market_cap: pair_data.market_cap,
                db_id: pair_data.db_id,
                state: pair_data.state
            };

            result.push(pair_contract_data);
        }
        result
    }


    fn get_pair(
        &self,
        first_token_id: TokenIdentifier,
        second_token_id: TokenIdentifier,
    ) -> ManagedAddress {
        let mut address = self
            .pair_map()
            .get(&PairTokens {
                first_token_id: first_token_id.clone(),
                second_token_id: second_token_id.clone(),
            })
            .unwrap_or_else(ManagedAddress::zero);

        if address.is_zero() {
            address = self
                .pair_map()
                .get(&PairTokens {
                    first_token_id: second_token_id,
                    second_token_id: first_token_id,
                })
                .unwrap_or_else(ManagedAddress::zero);
        }
        address
    }



}
