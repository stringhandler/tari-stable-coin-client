//  Copyright 2023 The Tari Project
//  SPDX-License-Identifier: BSD-3-Clause

use clap::Parser;
use clap::Subcommand;

#[derive(Parser, Debug)]
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
        default_value = "0xfd92dc534dbb9577bcc72a221acfac5c87bdb359ed0517ce44648fd5d028bf82"
    )]
    pub template: String,
    #[clap(long, short = 'd')]
    pub dump_buckets: bool,
    #[clap(long)]
    pub dry_run: bool,
    #[clap(subcommand)]
    pub command: Command,
    #[clap(long, short = 'f', default_value = "1000")]
    pub max_fee: u64,
    #[clap(long, short = 'a', default_value = "TestAccount_0")]
    pub default_account: String,
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

    RemoveFromBlacklist(remove_from_blacklist::Command),

    GetUserData(get_user_data::Command),

    SetUserData(set_user_data::Command),
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
    use clap::Args;

    use tari_engine_types::parse_arg;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    use tari_transaction::Transaction;

    use std::str::FromStr;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub account_component_address: String,
        pub admin_badge_resource: String,

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
            let method = "increase_supply".to_string();

            let instructions = Transaction::builder()
                .create_proof(
                    ComponentAddress::from_str(&self.account_component_address).unwrap(),
                    ResourceAddress::from_str(&self.admin_badge_resource).unwrap(),
                )
                .put_last_instruction_output_on_workspace("proof")
                .call_method(
                    ComponentAddress::from_str(&self.component_address).unwrap(),
                    "increase_supply",
                    args![123],
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
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
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

    use tari_utilities::hex::Hex;

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

    use tari_utilities::hex::Hex;

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
            let method = "total_supply".to_string();

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

pub(crate) mod withdraw {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    use std::str::FromStr;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::Transaction;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub account_component_address: String,
        pub user_badge_resource: String,
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
            let method = "withdraw".to_string();

            let instructions = Transaction::builder()
                .create_proof(
                    ComponentAddress::from_str(&self.account_component_address).unwrap(),
                    ResourceAddress::from_str(&self.user_badge_resource).unwrap(),
                )
                .put_last_instruction_output_on_workspace("proof")
                .call_method(
                    ComponentAddress::from_str(&self.component_address).unwrap(),
                    "withdraw",
                    args![self.amount.parse::<u64>().unwrap()],
                )
                .put_last_instruction_output_on_workspace("bucket")
                .call_method(
                    ComponentAddress::from_str(&self.account_component_address).unwrap(),
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
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
                )
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

    use tari_utilities::hex::Hex;

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

pub(crate) mod create_new_admin {
    use crate::daemon_client::DaemonClient;
    use clap::Args;

    use tari_engine_types::instruction::Instruction;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    use tari_utilities::hex::Hex;

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

    use tari_engine_types::instruction::Instruction;
    use tari_engine_types::parse_arg;

    use tari_template_lib::args;

    use tari_template_lib::prelude::ComponentAddress;

    use std::str::FromStr;
    use tari_template_lib::prelude::ResourceAddress;
    use tari_transaction::Transaction;
    use tari_utilities::hex::Hex;

    #[derive(Debug, Args, Clone)]
    pub struct Command {
        pub account_component_address: String,
        pub admin_badge_resource: String,

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
            let method = "create_new_user".to_string();

            let instructions = Transaction::builder()
                .create_proof(
                    ComponentAddress::from_str(&self.account_component_address).unwrap(),
                    ResourceAddress::from_str(&self.admin_badge_resource).unwrap(),
                )
                .put_last_instruction_output_on_workspace("proof")
                .call_method(
                    ComponentAddress::from_str(&self.component_address).unwrap(),
                    "create_new_user",
                    args![self.user_id.parse::<u64>().unwrap()],
                )
                .put_last_instruction_output_on_workspace("bucket")
                .call_method(
                    ComponentAddress::from_str(&self.account_component_address).unwrap(),
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
                    vec![format!("component_{}", self.component_address)
                        .parse()
                        .unwrap()],
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

    use tari_utilities::hex::Hex;

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

    use tari_utilities::hex::Hex;

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

    use tari_utilities::hex::Hex;

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
