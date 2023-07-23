use jsonrpsee::types::ErrorObjectOwned;
use papyrus_storage::body::BodyStorageReader;
use papyrus_storage::db::TransactionKind;
use papyrus_storage::StorageTxn;
use starknet_api::block::BlockNumber;

use crate::api::JsonRpcError;
use crate::internal_server_error;

pub fn get_block_txs_by_number<
    Mode: TransactionKind,
    Transaction: From<starknet_api::transaction::Transaction>,
>(
    txn: &StorageTxn<'_, Mode>,
    block_number: BlockNumber,
) -> Result<Vec<Transaction>, ErrorObjectOwned> {
    let transactions = txn
        .get_block_transactions(block_number)
        .map_err(internal_server_error)?
        .ok_or_else(|| ErrorObjectOwned::from(JsonRpcError::BlockNotFound))?;

    Ok(transactions.into_iter().map(Transaction::from).collect())
}
