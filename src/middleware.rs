use async_trait::async_trait;
use ethers::{
    prelude::Middleware,
    providers::{MiddlewareError, PendingTransaction},
    types::{transaction::eip2718::TypedTransaction, BlockId, Bytes},
};
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum BundlerMiddlewareError<M: Middleware> {
    #[error("Middleware Error: {0}")]
    MiddlewareError(M::Error),
}

impl<M: Middleware> MiddlewareError for BundlerMiddlewareError<M> {
    type Inner = M::Error;

    fn from_err(e: Self::Inner) -> Self {
        BundlerMiddlewareError::MiddlewareError(e)
    }

    fn as_inner(&self) -> Option<&Self::Inner> {
        match self {
            BundlerMiddlewareError::MiddlewareError(e) => Some(e),
        }
    }
}

/// Use this middleware to send transactions from an ERC4337 compliant smart contract via a compliant bundler
#[derive(Debug, Clone)]
struct BundlerMiddleware<M> {
    inner: M,
    bundler_url: Url,
}

impl<M: Middleware> BundlerMiddleware<M> {
    fn new(inner: M, bundler_url: impl Into<Url>) -> Self {
        Self {
            inner,
            bundler_url: bundler_url.into(),
        }
    }
}

#[async_trait]
impl<M: Middleware> Middleware for BundlerMiddleware<M> {
    type Error = BundlerMiddlewareError<M>;
    type Inner = M;
    type Provider = M::Provider;

    fn inner(&self) -> &M {
        &self.inner
    }

    /// This is triggered from SignerMiddleware if the signer is different compared to the from address in the provided transaction
    /// From will be the ERC4337 AA wallet, while the signer provided will be the owner of that ERC4337 wallet
    async fn send_transaction<T: Into<TypedTransaction> + Send + Sync>(
        &self,
        tx: T,
        block: Option<BlockId>,
    ) -> Result<PendingTransaction<'_, Self::Provider>, Self::Error> {
        println!("Sending transaction as a user op");

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use ethers::{
        middleware::SignerMiddleware,
        providers::{Http, Middleware, Provider},
        signers::LocalWallet,
        types::{
            transaction::eip2718::TypedTransaction, Address, Eip1559TransactionRequest,
            TransactionRequest,
        },
    };
    use tokio;
    use url::Url;

    use super::BundlerMiddleware;

    #[tokio::test]
    async fn test_bundler() {
        let aa_owner = "0x26d45bda70d0ed86247c9f701cce70e0574556d0fd1fc5fc32a69f84397ef768"
            .parse::<LocalWallet>()
            .unwrap();
        let client = SignerMiddleware::new(
            BundlerMiddleware::new(
                Provider::<Http>::try_from("http://localhost:8545").unwrap(),
                Url::parse("https://eth-sepolia.g.alchemy.com/v2/seslbW3Spoy_21yc1HHeUGsswR_EKHC_")
                    .unwrap(),
            ),
            aa_owner,
        );
        let tx = TypedTransaction::Eip1559(
            Eip1559TransactionRequest::new()
                .from(Address::random())
                .value(100)
                .to("vitalik.eth"),
        );
        println!("{:?}", tx);
        println!("{:?}", client.address());
        let pending_tx = client.send_transaction(tx, None).await.unwrap();
    }
}
