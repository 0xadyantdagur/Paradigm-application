use alloy_rpc_types::TransactionRequest;
use alloy_sol_macro::sol;

sol!(DepositContract, "src/contracts/deposit_contract.json");

/// proof of concept
/// makes the transaction that calls our contract to send 1 ETH to the game contract
pub fn make_transaction(block_number: u64) -> TransactionRequest {
    todo!()
}
