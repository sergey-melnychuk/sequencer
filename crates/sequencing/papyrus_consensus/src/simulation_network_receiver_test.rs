use futures::{SinkExt, StreamExt};
use papyrus_network::network_manager::test_utils::create_test_broadcasted_message_manager;
use papyrus_protobuf::consensus::ConsensusMessage;
use test_case::test_case;

use super::NetworkReceiver;

const CACHE_SIZE: usize = 10;
const SEED: u64 = 123;
const DROP_PROBABILITY: f64 = 0.5;
const INVALID_PROBABILITY: f64 = 0.5;

#[test_case(true, true; "distinct_vote")]
#[test_case(false, true; "repeat_vote")]
#[test_case(true, false; "distinct_proposal")]
#[test_case(false, false; "repeat_proposal")]
#[tokio::test]
async fn test_invalid(distinct_messages: bool, is_vote: bool) {
    let (mut sender, receiver) = futures::channel::mpsc::unbounded();
    let mut receiver = NetworkReceiver::new(receiver, CACHE_SIZE, SEED, 0.0, INVALID_PROBABILITY);
    let mut invalid_messages = 0;

    for height in 0..1000 {
        let msg = create_consensus_msg(if distinct_messages { height } else { 0 }, is_vote);
        let broadcasted_message_manager = create_test_broadcasted_message_manager();
        sender.send((Ok(msg.clone()), broadcasted_message_manager)).await.unwrap();
        if receiver.next().await.unwrap().0.unwrap() != msg {
            invalid_messages += 1;
        }
    }
    assert!((400..=600).contains(&invalid_messages), "num_invalid={invalid_messages}");
}

#[test_case(true, true; "distinct_vote")]
#[test_case(false, true; "repeat_vote")]
#[test_case(true, false; "distinct_proposal")]
#[test_case(false, false; "repeat_proposal")]
#[tokio::test]
async fn test_drops(distinct_messages: bool, is_vote: bool) {
    let (mut sender, receiver) = futures::channel::mpsc::unbounded();
    let mut receiver = NetworkReceiver::new(receiver, CACHE_SIZE, SEED, DROP_PROBABILITY, 0.0);
    let mut num_received = 0;

    for height in 0..1000 {
        let msg = create_consensus_msg(if distinct_messages { height } else { 0 }, is_vote);
        let broadcasted_message_manager = create_test_broadcasted_message_manager();
        sender.send((Ok(msg.clone()), broadcasted_message_manager)).await.unwrap();
    }
    drop(sender);

    while receiver.next().await.is_some() {
        num_received += 1;
    }
    assert!((400..=600).contains(&num_received), "num_received={num_received}");
}

fn create_consensus_msg(height: u64, is_vote: bool) -> ConsensusMessage {
    if is_vote {
        ConsensusMessage::Vote(papyrus_protobuf::consensus::Vote { height, ..Default::default() })
    } else {
        ConsensusMessage::Proposal(papyrus_protobuf::consensus::Proposal {
            height,
            ..Default::default()
        })
    }
}