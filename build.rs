// Copyright 2022 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure().build_client(true).build_server(true).compile(
        &[
            "proto/base_node.proto",
            "proto/block.proto",
            "proto/network.proto",
            "proto/p2pool.proto",
            "proto/sidechain_types.proto",
            "proto/transaction.proto",
            "proto/types.proto",
            "proto/wallet.proto",
            "proto/validator_node.proto",
            "proto/p2pool.proto",
        ],
        &["proto"],
    )?;

    Ok(())
}
