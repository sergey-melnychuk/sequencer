use assert_matches::assert_matches;

use crate::{
    shash, ClassHash, ContractAddress, ContractClass, ContractNonce, DeclaredContract,
    DeployedContract, Nonce, StarkHash, StarknetApiError, StateDiff, StorageDiff,
};

#[test]
fn state_sorted() {
    let hash0 = shash!("0x0");
    let hash1 = shash!("0x1");

    let dep_contract_0 = DeployedContract {
        address: ContractAddress::try_from(hash0).unwrap(),
        class_hash: ClassHash::new(hash0),
    };
    let dep_contract_1 = DeployedContract {
        address: ContractAddress::try_from(hash1).unwrap(),
        class_hash: ClassHash::new(hash1),
    };
    let storage_diff_0 =
        StorageDiff { address: ContractAddress::try_from(hash0).unwrap(), storage_entries: vec![] };
    let storage_diff_1 =
        StorageDiff { address: ContractAddress::try_from(hash1).unwrap(), storage_entries: vec![] };

    let dec_contract_0 = DeclaredContract {
        class_hash: ClassHash::new(hash0),
        contract_class: ContractClass::default(),
    };
    let dec_contract_1 = DeclaredContract {
        class_hash: ClassHash::new(hash1),
        contract_class: ContractClass::default(),
    };

    let nonce_0 = ContractNonce {
        contract_address: ContractAddress::try_from(hash0).unwrap(),
        nonce: Nonce::new(hash0),
    };
    let nonce_1 = ContractNonce {
        contract_address: ContractAddress::try_from(hash1).unwrap(),
        nonce: Nonce::new(hash1),
    };

    let unsorted_deployed_contracts = vec![dep_contract_1.clone(), dep_contract_0.clone()];
    let unsorted_storage_diffs = vec![storage_diff_1.clone(), storage_diff_0.clone()];
    let unsorted_declared_contracts = vec![dec_contract_1.clone(), dec_contract_0.clone()];
    let unsorted_nonces = vec![nonce_1.clone(), nonce_0.clone()];

    let state_diff = StateDiff::new(
        unsorted_deployed_contracts,
        unsorted_storage_diffs,
        unsorted_declared_contracts,
        unsorted_nonces,
    )
    .unwrap();

    let sorted_deployed_contracts = vec![dep_contract_0, dep_contract_1];
    let sorted_storage_diffs = vec![storage_diff_0, storage_diff_1];
    let sorted_declared_contracts = vec![dec_contract_0, dec_contract_1];
    let sorted_nonces = vec![nonce_0, nonce_1];

    assert_eq!(
        state_diff.destruct(),
        (sorted_deployed_contracts, sorted_storage_diffs, sorted_declared_contracts, sorted_nonces)
    );
}

#[test]
fn state_unique() {
    let hash0 = shash!("0x0");

    let dep_contract = DeployedContract {
        address: ContractAddress::try_from(hash0).unwrap(),
        class_hash: ClassHash::new(hash0),
    };

    let storage_diff =
        StorageDiff { address: ContractAddress::try_from(hash0).unwrap(), storage_entries: vec![] };

    let dec_contract = DeclaredContract {
        class_hash: ClassHash::new(hash0),
        contract_class: ContractClass::default(),
    };

    let nonce = ContractNonce {
        contract_address: ContractAddress::try_from(hash0).unwrap(),
        nonce: Nonce::new(hash0),
    };

    let hash1 = shash!("0x1");
    let deployed_contract_duplicate = DeployedContract {
        address: ContractAddress::try_from(hash0).unwrap(),
        class_hash: ClassHash::new(hash1),
    };
    let declared_contract_duplicate = DeclaredContract {
        class_hash: ClassHash::new(hash0),
        contract_class: ContractClass::default(),
    };

    let state_diff_with_duplicate_deployed_contract = StateDiff::new(
        vec![dep_contract.clone(), deployed_contract_duplicate],
        vec![storage_diff.clone()],
        vec![dec_contract.clone()],
        vec![nonce.clone()],
    );
    assert_matches!(state_diff_with_duplicate_deployed_contract, Err(StarknetApiError::DuplicateInStateDiff{object}) if object == "deployed_contracts");

    let state_diff_with_duplicate_declared_contract = StateDiff::new(
        vec![dep_contract],
        vec![storage_diff],
        vec![dec_contract, declared_contract_duplicate],
        vec![nonce],
    );
    assert_matches!(state_diff_with_duplicate_declared_contract, Err(StarknetApiError::DuplicateInStateDiff{object}) if object == "declared_contracts");
}
