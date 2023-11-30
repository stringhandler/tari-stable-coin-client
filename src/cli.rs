//  Copyright 2023 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
pub(crate) struct Cli {
    #[clap(long, short = 'e', alias = "endpoint", env = "JRPC_ENDPOINT")]
    pub daemon_jrpc_endpoint: Option<String>,
    #[clap(long, short = 't', alias = "token")]
    pub auth_token: Option<String>,
    #[clap(
        long,
        alias = "template_address",
        default_value = "0x6c658001d3c8587b194990ea0ffd643c38685d5d7d13ef398ac62875c79fbab4"
    )]
    pub template: String,
    #[clap(long, short = 'd')]
    pub dump_buckets: bool,
    #[clap(long)]
    pub dry_run: bool,
    #[clap(subcommand)]
    pub command: Command,
    #[clap(long, short = 'f', default_value = "1500")]
    pub max_fee: u64,
    #[clap(long, short = 'a', default_value = "TestAccount_0")]
    pub default_account: String,
    #[clap(
        long,
        default_value = "component_ea50fa2002a7898eef13a851ed41cacc713b6bf15c5ab4530d1ac5acc4d06263"
    )]
    pub default_coin_component: String,
    #[clap(
        long,
        default_value = "resource_bc5ab4d0974aa8347d5060cf20f9f0b0b415bfb45559f6d94e913d1cfb04b614"
    )]
    pub admin_badge_resource: String,
    #[clap(
        long,
        default_value = "resource_d7d57fd8a795d243f0d93d4e7e651bb210a0f0f990ceb242712b56f926b636e0"
    )]
    pub user_badge_resource: String,
    #[clap(
        long,
        default_value = "resource_7af49ffcb972d90dd04a29ee32c099b46140d8c0422a7200c0c3118636a75f0d"
    )]
    pub coin_resource: String,
}

impl Cli {
    pub fn init() -> Self {
        Self::parse()
    }
}

#[derive(Debug, Subcommand, Clone)]
pub(crate) enum Command {
    Login(login::Command),

    Instantiate(instantiate::Command),

    IncreaseSupply(increase_supply::Command),

    DecreaseSupply(decrease_supply::Command),

    TotalSupply(total_supply::Command),

    Withdraw(withdraw::Command),

    Deposit(deposit::Command),

    CreateNewAdmin(create_new_admin::Command),

    CreateNewUser(create_new_user::Command),

    BlacklistUser(blacklist_user::Command),

    RemoveFromBlacklist(remove_from_blacklist::Command),

    GetUserData(get_user_data::Command),

    SetUserData(set_user_data::Command),

    Send(send::Command),
}

pub mod login {
    use crate::daemon_client::DaemonClient;
    use clap::Args;
    use std::fs;

    #[derive(Debug, Args, Clone)]
    pub struct Command {}

    impl Command {
        pub async fn run(self, mut client: DaemonClient) {
            let token = client.login().await;
            fs::write("token.data", token).unwrap();
        }
    }
}

pub(crate) mod instantiate {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;
    use tari_engine_types::TemplateAddress;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub initial_token_supply: String,

        pub token_symbol: String,

        pub token_metadata: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            template_address: TemplateAddress,
            dump_buckets: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let function = "instantiate".to_string();

            client
                .submit_instruction(
                    Instruction::CallFunction {
                        template_address,
                        function,
                        args: vec![
                            parse_arg(&self.initial_token_supply).unwrap(),
                            parse_arg(&self.token_symbol).unwrap(),
                            parse_arg(&self.token_metadata).unwrap(),
                        ],
                    },
                    dump_buckets,
                    false,
                    fees,
                    vec![],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod increase_supply {
    use crate::daemon_client::DaemonClient;
    use crate::Cli;
    use clap::Args;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    use tari_transaction::Transaction;

    use std::str::FromStr;
    use tari_template_lib::prelude::ResourceAddress;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub account_component_address: String,
        pub amount: u64,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
            cli: Cli,
        ) {
            // let template_address= ;

            let instructions = Transaction::builder()
                .create_proof(
                    ComponentAddress::from_str(&self.account_component_address).unwrap(),
                    ResourceAddress::from_str(&cli.admin_badge_resource).unwrap(),
                )
                .put_last_instruction_output_on_workspace("proof")
                .call_method(
                    ComponentAddress::from_str(&cli.default_coin_component).unwrap(),
                    "increase_supply",
                    args![self.amount],
                )
                .drop_all_proofs_in_workspace()
                .build_as_instructions();

            // .
            // let mut instructions = vec![];

            // instructions.push(Instruction::CallMethod {
            // component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
            // method,
            // args: args![parse_arg(&self.amount).unwrap(),],
            // });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![cli.default_coin_component.parse().unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod decrease_supply {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub amount: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "decrease_supply".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![parse_arg(&self.amount).unwrap(),],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod total_supply {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_engine_types::instruction::Instruction;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    use crate::Cli;
    use std::str::FromStr;

    #[derive(Debug, Args, Clone)]
    pub struct Command {}

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
            cli: Cli,
        ) {
            // let template_address= ;
            let method = "total_supply".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_str(&cli.default_coin_component).unwrap(),
                method,
                args: args![],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![cli.default_coin_component.parse().unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod withdraw {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    use crate::Cli;
    use std::str::FromStr;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::Transaction;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub admin_account_component: String,
        pub into_account: String,
        pub amount: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
            cli: Cli,
        ) {
            // let template_address= ;

            let instructions = Transaction::builder()
                .create_proof(
                    ComponentAddress::from_str(&self.admin_account_component).unwrap(),
                    ResourceAddress::from_str(&cli.admin_badge_resource).unwrap(),
                )
                .put_last_instruction_output_on_workspace("proof")
                .call_method(
                    ComponentAddress::from_str(&cli.default_coin_component).unwrap(),
                    "withdraw",
                    args![self.amount.parse::<u64>().unwrap()],
                )
                .put_last_instruction_output_on_workspace("bucket")
                .call_method(
                    ComponentAddress::from_str(&self.into_account).unwrap(),
                    "deposit",
                    args![Variable("bucket"),],
                )
                .drop_all_proofs_in_workspace()
                .build_as_instructions();

            client
                .submit_instructions(instructions, dump_buckets, is_dry_run, fees, vec![])
                .await;
            println!("done");
        }
    }
}

pub(crate) mod deposit {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use std::str::FromStr;
    use tari_engine_types::instruction::Instruction;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;
    use tari_template_lib::prelude::ResourceAddress;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub bucket_amount: u64,
        pub bucket_resource: String,
        pub bucket_withdraw_from_component: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "deposit".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.bucket_withdraw_from_component)
                    .unwrap(),
                method: "withdraw".to_string(),
                args: args![
                    ResourceAddress::from_str(&self.bucket_resource).unwrap(),
                    self.bucket_amount
                ],
            });
            instructions.push(Instruction::PutLastInstructionOutputOnWorkspace {
                key: b"bucket_bucket".to_vec(),
            });

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![Variable("bucket_bucket"),],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod send {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    use crate::Cli;
    use std::str::FromStr;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::Transaction;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub from_component: String,
        pub to_component: String,
        pub amount: u64,
    }

    impl Command {
        pub async fn run(self, mut client: DaemonClient, is_dry_run: bool, fees: u64, cli: Cli) {
            let instructions = Transaction::builder()
                .create_proof(
                    ComponentAddress::from_str(&self.from_component).unwrap(),
                    ResourceAddress::from_str(&cli.user_badge_resource).unwrap(),
                )
                .put_last_instruction_output_on_workspace("proof")
                .call_method(
                    ComponentAddress::from_str(&self.from_component).unwrap(),
                    "withdraw",
                    args![
                        ResourceAddress::from_str(&cli.coin_resource)
                            .expect("bad resource address"),
                        self.amount
                    ],
                )
                .put_last_instruction_output_on_workspace("bucket")
                .call_method(
                    ComponentAddress::from_str(&self.to_component).unwrap(),
                    "deposit",
                    args![Variable("bucket"),],
                )
                .drop_all_proofs_in_workspace()
                .build_as_instructions();

            client
                .submit_instructions(
                    instructions,
                    false,
                    is_dry_run,
                    fees,
                    vec![cli.default_coin_component.parse().unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod create_new_admin {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_engine_types::instruction::Instruction;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "create_new_admin".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod create_new_user {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    use crate::Cli;
    use std::str::FromStr;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::Transaction;
    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub admin_account_component: String,
        pub user_id: u64,
        pub send_to_user_component: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
            cli: Cli,
        ) {
            // let template_address= ;

            let instructions = Transaction::builder()
                .create_proof(
                    ComponentAddress::from_str(&self.admin_account_component).unwrap(),
                    ResourceAddress::from_str(&cli.admin_badge_resource).unwrap(),
                )
                .put_last_instruction_output_on_workspace("proof")
                .call_method(
                    ComponentAddress::from_str(&cli.default_coin_component).unwrap(),
                    "create_new_user",
                    args![self.user_id],
                )
                .put_last_instruction_output_on_workspace("bucket")
                .call_method(
                    ComponentAddress::from_str(&self.send_to_user_component).unwrap(),
                    "deposit",
                    args![Variable("bucket"),],
                )
                .drop_all_proofs_in_workspace()
                .build_as_instructions();

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![cli.default_coin_component.parse().unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod blacklist_user {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_template_lib::args;

    use tari_template_lib::models::NonFungibleAddress;
    use tari_template_lib::prelude::ComponentAddress;
    use tari_transaction::SubstateRequirement;

    use crate::Cli;
    use std::str::FromStr;
    use tari_engine_types::parse_arg;
    use tari_engine_types::substate::SubstateAddress;
    use tari_template_lib::prelude::NonFungibleId;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_template_lib::prelude::VaultId;
    use tari_transaction::Transaction;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub admin_account_component: String,
        pub from_vault: String,
        pub user_id: u64,
    }

    impl Command {
        pub async fn run(self, mut client: DaemonClient, is_dry_run: bool, fees: u64, cli: Cli) {
            // let template_address= ;

            let instructions = Transaction::builder()
                .create_proof(
                    ComponentAddress::from_str(&self.admin_account_component).unwrap(),
                    ResourceAddress::from_str(&cli.admin_badge_resource).unwrap(),
                )
                .put_last_instruction_output_on_workspace("proof")
                .call_method(
                    ComponentAddress::from_str(&cli.default_coin_component).unwrap(),
                    "blacklist_user",
                    args![VaultId::from_hex(&self.from_vault).unwrap(), self.user_id],
                )
                .drop_all_proofs_in_workspace()
                .build_as_instructions();

            client
                .submit_instructions(
                    instructions,
                    false,
                    is_dry_run,
                    fees,
                    vec![
                        format!("vault_{}", self.from_vault).parse().unwrap(),
                        SubstateRequirement::new(
                            SubstateAddress::NonFungible(NonFungibleAddress::new(
                                ResourceAddress::from_str(&cli.user_badge_resource).unwrap(),
                                NonFungibleId::from_u64(self.user_id),
                            )),
                            None,
                        ),
                    ],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod remove_from_blacklist {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub user_id: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "remove_from_blacklist".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![parse_arg(&self.user_id).unwrap(),],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod get_user_data {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub user_id: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "get_user_data".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![parse_arg(&self.user_id).unwrap(),],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}

pub(crate) mod set_user_data {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub component_address: String,

        pub user_id: String,

        pub data: String,
    }

    impl Command {
        pub async fn run(
            self,
            mut client: DaemonClient,
            dump_buckets: bool,
            is_dry_run: bool,
            fees: u64,
        ) {
            // let template_address= ;
            let method = "set_user_data".to_string();

            let mut instructions = vec![];

            instructions.push(Instruction::CallMethod {
                component_address: ComponentAddress::from_hex(&self.component_address).unwrap(),
                method,
                args: args![
                    parse_arg(&self.user_id).unwrap(),
                    parse_arg(&self.data).unwrap(),
                ],
            });

            client
                .submit_instructions(
                    instructions,
                    dump_buckets,
                    is_dry_run,
                    fees,
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
                .await;
            println!("done");
        }
    }
}
