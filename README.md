# Tari Stablecoin Example Client

This client was generated by tari_scaffolder, and then modified. It's a work in progress so some method may not work.

The template, default component and resources are set in the Cli struct. If you deploy a new contract, you should update these
defaults.

To run this locally, you need to run a `tari_dan_wallet_daemon` and it needs to be connected to a running `tari_indexer` that is connected to 
the Tari DAN testnet.

When starting, you can specify the wallet's JRPC using `-e http://localhost:9000 ` or changing the `daemon_jrpc_endpoint` CLI option.

You will also need a JWT token to interact with the DAN wallet. At some point I will add logging in functionality, but for now 
you should copy the JWT from the DAN wallet user interface. I usually open the DAN UI (http://localhost:5100/) and then developer tools
and copy the JWT from the Bearer Auth header. Copy this token into token.data in the root of this project.

## Interacting

Sending can be done via:
`cargo run -- -e <daemon_endpoint> -a <Account name in Wallet> send <from_account_component_address> <to_account> <amount>`

e.g.
`cargo run -- -e http://localhost:9000 -a asdf send component_a266bd051058772a03edd8bc0503c8021b278f03bb60a619a077c3fdc9141f74 component_763ae73e696169153d2058d882cc3337926ef2fd1e0e318a05bbb4cac7b7e5ca 1`

TODO:
- Get registrations
- blacklist people
- trace transactions
- deploy indexer
- martin rfcs 
- look at priorities on board

