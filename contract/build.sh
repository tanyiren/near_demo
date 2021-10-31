#!/bin/bash
set -e
rustup target add wasm32-unknown-unknown
RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
# RUSTFLAGS='-C link-arg=-s' cargo +stable build --target wasm32-unknown-unknown --release
#cp target/wasm32-unknown-unknown/release/ref_farming.wasm ./res/ref_farming_local.wasm
#near dev-deploy --wasmFile=target/wasm32-unknown-unknown/release/greeter.wasm
# near dev-deploy --wasmFile=target/wasm32-unknown-unknown/release/greeter.wasm 



# near login
# near dev-deploy --wasmFile=target/wasm32-unknown-unknown/release/greeter.wasm 
# ID=dev-1635679474997-60584759748943
# near call $ID new --accountId=$ID
# near call $ID set_award '{"account":\'$ID\',"round":1}' --accountId=$ID
# echo $ID
# near call $ID set_award '{"account":"dev-1635679474997-60584759748943","round":1}' --accountId=$ID
# near call $ID user_bet '{"account_id":"ok3.testnet","stake_num":2}' --accountId=$ID
# near call $ID user_bet '{"account_id":"ok3.testnet","stake_num":2}' --accountId=$ID --deposit=0.0000000001
# near call $ID get_award '{"account":"dev-1635679474997-60584759748943"}'
# near call $ID get_award '{"account":"dev-1635679474997-60584759748943"}' --accountId=$ID
# near call $ID get_award_history --accountId=$ID
# near call $ID get_award_history --accountId=$ID 
# near call $ID is_round_winner '{"account":"ok3.testnet","round":1}' --accountId=$ID