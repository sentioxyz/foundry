use foundry_compilers::artifacts::{BytecodeHash, EvmVersion};
use foundry_config::Config;
use foundry_test_utils::{
    forgetest_async,
    rpc::{next_etherscan_api_key, next_http_archive_rpc_endpoint},
    util::OutputExt,
    TestCommand, TestProject,
};

fn test_verify_bytecode(
    prj: TestProject,
    mut cmd: TestCommand,
    addr: &str,
    contract_name: &str,
    config: Config,
    verifier: &str,
    verifier_url: &str,
    expected_matches: (&str, &str),
) {
    let etherscan_key = next_etherscan_api_key();
    let rpc_url = next_http_archive_rpc_endpoint();

    // fetch and flatten source code
    let source_code = cmd
        .cast_fuse()
        .args(["etherscan-source", addr, "--flatten", "--etherscan-api-key", &etherscan_key])
        .assert_success()
        .get_output()
        .stdout_lossy();

    prj.add_source(contract_name, &source_code).unwrap();
    prj.write_config(config);

    let output = cmd
        .forge_fuse()
        .args([
            "verify-bytecode",
            addr,
            contract_name,
            "--etherscan-api-key",
            &etherscan_key,
            "--verifier",
            verifier,
            "--verifier-url",
            verifier_url,
            "--rpc-url",
            &rpc_url,
        ])
        .assert_success()
        .get_output()
        .stdout_lossy();

    assert!(output
        .contains(format!("Creation code matched with status {}", expected_matches.0).as_str()));
    assert!(output
        .contains(format!("Runtime code matched with status {}", expected_matches.1).as_str()));
}

fn test_verify_bytecode_with_ignore(
    prj: TestProject,
    mut cmd: TestCommand,
    addr: &str,
    contract_name: &str,
    config: Config,
    verifier: &str,
    verifier_url: &str,
    expected_matches: (&str, &str),
    ignore: &str,
    chain: &str,
) {
    let etherscan_key = next_etherscan_api_key();
    let rpc_url = next_http_archive_rpc_endpoint();

    // fetch and flatten source code
    let source_code = cmd
        .cast_fuse()
        .args([
            "etherscan-source",
            addr,
            "--flatten",
            "--etherscan-api-key",
            &etherscan_key,
            "--chain",
            chain,
        ])
        .assert_success()
        .get_output()
        .stdout_lossy();

    prj.add_source(contract_name, &source_code).unwrap();
    prj.write_config(config);

    let output = cmd
        .forge_fuse()
        .args([
            "verify-bytecode",
            addr,
            contract_name,
            "--etherscan-api-key",
            &etherscan_key,
            "--verifier",
            verifier,
            "--verifier-url",
            verifier_url,
            "--rpc-url",
            &rpc_url,
            "--ignore",
            ignore,
        ])
        .assert_success()
        .get_output()
        .stdout_lossy();

    if ignore == "creation" {
        assert!(!output.contains(
            format!("Creation code matched with status {}", expected_matches.0).as_str()
        ));
    } else {
        assert!(output.contains(
            format!("Creation code matched with status {}", expected_matches.0).as_str()
        ));
    }

    if ignore == "runtime" {
        assert!(!output
            .contains(format!("Runtime code matched with status {}", expected_matches.1).as_str()));
    } else {
        assert!(output
            .contains(format!("Runtime code matched with status {}", expected_matches.1).as_str()));
    }
}
forgetest_async!(can_verify_bytecode_no_metadata, |prj, cmd| {
    test_verify_bytecode(
        prj,
        cmd,
        "0xba2492e52F45651B60B8B38d4Ea5E2390C64Ffb1",
        "SystemConfig",
        Config {
            evm_version: EvmVersion::London,
            optimizer_runs: 999999,
            optimizer: true,
            cbor_metadata: false,
            bytecode_hash: BytecodeHash::None,
            ..Default::default()
        },
        "etherscan",
        "https://api.etherscan.io/api",
        ("full", "full"),
    );
});

forgetest_async!(can_verify_bytecode_with_metadata, |prj, cmd| {
    test_verify_bytecode(
        prj,
        cmd,
        "0xb8901acb165ed027e32754e0ffe830802919727f",
        "L1_ETH_Bridge",
        Config {
            evm_version: EvmVersion::Paris,
            optimizer_runs: 50000,
            optimizer: true,
            ..Default::default()
        },
        "etherscan",
        "https://api.etherscan.io/api",
        ("partial", "partial"),
    );
});

// Test non-CREATE2 deployed contract with blockscout
forgetest_async!(can_verify_bytecode_with_blockscout, |prj, cmd| {
    test_verify_bytecode(
        prj,
        cmd,
        "0x70f44C13944d49a236E3cD7a94f48f5daB6C619b",
        "StrategyManager",
        Config {
            evm_version: EvmVersion::London,
            optimizer: true,
            optimizer_runs: 200,
            ..Default::default()
        },
        "blockscout",
        "https://eth.blockscout.com/api",
        ("partial", "partial"),
    );
});

// Test CREATE2 deployed contract with blockscout
forgetest_async!(can_vb_create2_with_blockscout, |prj, cmd| {
    test_verify_bytecode(
        prj,
        cmd,
        "0xba2492e52F45651B60B8B38d4Ea5E2390C64Ffb1",
        "SystemConfig",
        Config {
            evm_version: EvmVersion::London,
            optimizer_runs: 999999,
            optimizer: true,
            cbor_metadata: false,
            bytecode_hash: BytecodeHash::None,
            ..Default::default()
        },
        "blockscout",
        "https://eth.blockscout.com/api",
        ("full", "full"),
    );
});

// `--ignore` tests
forgetest_async!(can_ignore_creation, |prj, cmd| {
    test_verify_bytecode_with_ignore(
        prj,
        cmd,
        "0xba2492e52F45651B60B8B38d4Ea5E2390C64Ffb1",
        "SystemConfig",
        Config {
            evm_version: EvmVersion::London,
            optimizer_runs: 999999,
            optimizer: true,
            cbor_metadata: false,
            bytecode_hash: BytecodeHash::None,
            ..Default::default()
        },
        "etherscan",
        "https://api.etherscan.io/api",
        ("full", "full"),
        "creation",
        "1",
    );
});

forgetest_async!(can_ignore_runtime, |prj, cmd| {
    test_verify_bytecode_with_ignore(
        prj,
        cmd,
        "0xba2492e52F45651B60B8B38d4Ea5E2390C64Ffb1",
        "SystemConfig",
        Config {
            evm_version: EvmVersion::London,
            optimizer_runs: 999999,
            optimizer: true,
            cbor_metadata: false,
            bytecode_hash: BytecodeHash::None,
            ..Default::default()
        },
        "etherscan",
        "https://api.etherscan.io/api",
        ("full", "ignored"),
        "runtime",
        "1",
    );
});

// Test predeploy contracts
// TODO: Add test utils for base such as basescan keys and alchemy keys.
// WETH9 Predeploy
// forgetest_async!(can_verify_predeploys, |prj, cmd| {
//     test_verify_bytecode_with_ignore(
//         prj,
//         cmd,
//         "0x4200000000000000000000000000000000000006",
//         "WETH9",
//         Config {
//             evm_version: EvmVersion::default(),
//             optimizer: true,
//             optimizer_runs: 10000,
//             cbor_metadata: true,
//             bytecode_hash: BytecodeHash::Bzzr1,
//             ..Default::default()
//         },
//         "etherscan",
//         "https://api.basescan.org/api",
//         ("ignored", "partial"),
//         "creation",
//         "base",
//     );
// });
