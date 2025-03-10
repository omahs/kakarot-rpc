// //! Kakarot RPC module for Ethereum.
// //! It is an adapter layer to interact with Kakarot ZK-EVM.
use std::net::{AddrParseError, SocketAddr};
pub mod eth_rpc;
use eth_rpc::{EthApiServer, KakarotEthRpc};
use eyre::Result;
use jsonrpsee::server::{ServerBuilder, ServerHandle};
use kakarot_rpc_core::lightclient::StarknetClient;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpcError {
    #[error(transparent)]
    JsonRpcServerError(#[from] jsonrpsee::core::Error),
    #[error(transparent)]
    ParseError(#[from] AddrParseError),
}

pub async fn run_server(
    starknet_client: Box<dyn StarknetClient>,
) -> Result<(SocketAddr, ServerHandle), RpcError> {
    let server = ServerBuilder::default()
        .build("127.0.0.1:03030".parse::<SocketAddr>()?)
        .await?;

    let addr = server.local_addr()?;

    let rpc_calls = KakarotEthRpc::new(starknet_client);
    let handle = server.start(rpc_calls.into_rpc())?;

    Ok((addr, handle))
}
