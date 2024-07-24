use std::{collections::HashSet, future::Future, pin::Pin, task::Poll};

use crate::contracts::make_transaction;
use alloy_primitives::{Address, Bytes, TxHash, U256};
use alloy_provider::{Provider, RootProvider};
use alloy_pubsub::{PubSubFrontend, Subscription};
use alloy_rpc_client::{ClientBuilder, WsConnect};
use alloy_rpc_types::{
    Block, BlockId, Filter, Log, Rich, Transaction, TransactionReceipt, TransactionRequest,
};

use futures::StreamExt;
use futures::{future::join_all, stream::FuturesUnordered, Stream};
use std::sync::Arc;
use std::task::Context;

pub struct EthPubSubClient {
    owner_address: Address,
    provider: Arc<RootProvider<PubSubFrontend>>,
    block_stream: Pin<Box<dyn Stream<Item = Block>>>,
    futs: FuturesUnordered<Pin<Box<dyn Future<Output = eyre::Result<bool>>>>>,
    contract_won: bool,
}

impl EthPubSubClient {
    pub async fn new_ws(ws_url: String, owner_address: Address) -> eyre::Result<Self> {
        let builder = ClientBuilder::default().ws(WsConnect::new(ws_url)).await?;
        let provider = Arc::new(RootProvider::new(builder));
        let block_stream = Box::pin(provider.subscribe_blocks().await?.into_stream());
        Ok(Self {
            owner_address,
            provider,
            block_stream,
            futs: FuturesUnordered::new(),
            contract_won: false,
        })
    }

    pub fn provider(&self) -> Arc<RootProvider<PubSubFrontend>> {
        self.provider.clone()
    }

    fn handle_poll(&mut self, cx: &mut Context<'_>) -> bool {
        if !self.contract_won {
            while let Poll::Ready(Some(block)) = self.block_stream.poll_next_unpin(cx) {
                let owner_address = self.owner_address;
                if let Some(block_num) = block.header.number {
                    let tx = make_transaction(block_num);
                    let provider = self.provider();
                    self.futs
                        .push(Box::pin(Self::send_tx(provider, owner_address, tx)))
                }

                self.futs.push(Box::pin(Self::check_if_won(owner_address)));
            }
        }

        while let Poll::Ready(Some(_)) = self.futs.poll_next_unpin(cx) {}

        self.futs.is_empty() && self.contract_won
    }

    /// sends the tx to our contract to send 1 ETH to the game contract
    async fn send_tx(
        provider: Arc<RootProvider<PubSubFrontend>>,
        creator_address: Address,
        tx: TransactionRequest,
    ) -> eyre::Result<bool> {
        if !Self::assert_contract_balance(&provider, creator_address).await? {
            return Err(eyre::eyre!("balance is < 1 ETH, cannot send transaction"));
        }

        provider.send_transaction(tx).await?;

        Ok(true)
    }

    /// checks the balance of our contract to ensure we have >=1 ETH to send to the game contract
    async fn assert_contract_balance(
        provider: &RootProvider<PubSubFrontend>,
        creator_address: Address,
    ) -> eyre::Result<bool> {
        if provider
            .get_balance(creator_address, BlockId::latest())
            .await?
            >= U256::from(1u8).pow(U256::from(18u64))
        {
            return Ok(true);
        } else {
            Ok(false)
        }
    }

    /// checks to see if our contract won the contest
    async fn check_if_won(owner_address: Address) -> eyre::Result<bool> {
        let won_contest: bool = todo!();

        if won_contest {
            Self::repay_depositors(owner_address).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// repays the depositors of the contract
    async fn repay_depositors(owner_address: Address) -> eyre::Result<()> {
        todo!();
    }
}

impl Future for EthPubSubClient {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if this.handle_poll(cx) {
            return Poll::Ready(());
        } else {
            Poll::Pending
        }
    }
}
