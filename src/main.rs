//  Copyright 2023 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

mod cli;
mod daemon_client;

use crate::cli::Cli;
use crate::cli::Command;
use crate::daemon_client::DaemonClient;

use std::fs;
use tari_utilities::hex::from_hex;

#[tokio::main]
async fn main() {
    let cli = Cli::init();
    let jrpc = cli
        .daemon_jrpc_endpoint
        .clone()
        .unwrap_or_else(|| "http://127.0.0.1:18016".to_string());
    let token = cli
        .auth_token
        .as_ref()
        .map(|a| a.to_string())
        .or(fs::read_to_string("token.data").ok());

    let client = DaemonClient::new(jrpc, token, cli.default_account.clone());
    let template_address = from_hex(&cli.template).unwrap().try_into().unwrap();
    let cli_clone_hack = cli.clone();
    match cli.command {
        Command::Login(com) => {
            com.run(client).await;
        }

        Command::Instantiate(com) => {
            com.run(client, template_address, cli.dump_buckets, cli.max_fee)
                .await;
        }

        Command::IncreaseSupply(com) => {
            com.run(
                client,
                cli.dump_buckets,
                cli.dry_run,
                cli.max_fee,
                cli_clone_hack,
            )
            .await;
        }

        Command::DecreaseSupply(com) => {
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee)
                .await;
        }

        Command::TotalSupply(com) => {
            com.run(
                client,
                cli.dump_buckets,
                cli.dry_run,
                cli.max_fee,
                cli_clone_hack,
            )
            .await;
        }

        Command::Withdraw(com) => {
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee)
                .await;
        }

        Command::Deposit(com) => {
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee)
                .await;
        }

        Command::CreateNewAdmin(com) => {
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee)
                .await;
        }

        Command::CreateNewUser(com) => {
            com.run(
                client,
                cli.dump_buckets,
                cli.dry_run,
                cli.max_fee,
                cli_clone_hack,
            )
            .await;
        }

        Command::RemoveFromBlacklist(com) => {
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee)
                .await;
        }

        Command::GetUserData(com) => {
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee)
                .await;
        }

        Command::SetUserData(com) => {
            com.run(client, cli.dump_buckets, cli.dry_run, cli.max_fee)
                .await;
        }
        Command::Send(com) => {
            com.run(client, cli.dry_run, cli.max_fee, cli_clone_hack)
                .await;
        }
    }
}
