use std::clone::Clone;

use crate::protocols::mediated_connection::invitee::states::requested::RequestedState;
use crate::protocols::mediated_connection::invitee::states::responded::RespondedState;
use messages::diddoc::aries::diddoc::AriesDidDoc;
use messages::protocols::connection::response::Response;
use messages::protocols::discovery::disclose::ProtocolDescriptor;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompleteState {
    pub did_doc: AriesDidDoc,
    pub bootstrap_did_doc: AriesDidDoc,
    pub protocols: Option<Vec<ProtocolDescriptor>>,
}

impl From<(CompleteState, Vec<ProtocolDescriptor>)> for CompleteState {
    fn from((state, protocols): (CompleteState, Vec<ProtocolDescriptor>)) -> CompleteState {
        trace!("ConnectionInvitee: transit state from CompleteState to CompleteState");
        CompleteState {
            bootstrap_did_doc: state.bootstrap_did_doc,
            did_doc: state.did_doc,
            protocols: Some(protocols),
        }
    }
}

impl From<(RequestedState, Response)> for CompleteState {
    fn from((state, response): (RequestedState, Response)) -> CompleteState {
        trace!("ConnectionInvitee: transit state from RequestedState to CompleteState");
        CompleteState {
            bootstrap_did_doc: state.did_doc,
            did_doc: response.connection.did_doc,
            protocols: None,
        }
    }
}

impl From<RespondedState> for CompleteState {
    fn from(state: RespondedState) -> CompleteState {
        trace!("ConnectionInvitee: transit state from RespondedState to CompleteState");
        CompleteState {
            bootstrap_did_doc: state.did_doc,
            did_doc: state.response.connection.did_doc,
            protocols: None,
        }
    }
}