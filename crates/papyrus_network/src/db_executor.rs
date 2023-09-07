use futures::channel::mpsc::UnboundedReceiver;
use futures::channel::oneshot;
#[cfg(test)]
use mockall::automock;

use crate::BlocksRange;

#[derive(thiserror::Error)]
pub enum ReaderError {}

#[derive(thiserror::Error)]
pub enum BlockError {}

pub struct ReaderCommunication<Response> {
    pub result_receiver: UnboundedReceiver<Response>,
    pub error_receiver: UnboundedReceiver<BlockError>,
    pub is_finished: oneshot::Receiver<Result<(), ReaderError>>,
}

#[cfg_attr(test, automock)]
pub trait ReaderExecutor<Response> {
    fn start_reading(&self, blocks_range: BlocksRange) -> ReaderCommunication<Response>;
}
