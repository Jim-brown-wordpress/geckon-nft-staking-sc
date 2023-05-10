##### - configuration - #####
PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"

WALLET="./wallets/devnet.pem"

################################################
OLD_NFT_TOKEN_ID="CITYNFT-150731"
OLD_NFT_TOKEN_ID_HEX="0x$(echo -n ${OLD_NFT_TOKEN_ID} | xxd -p -u | tr -d '\n')"
OLD_NFT_TOKEN_ID_ONLY_HEX="$(echo -n ${OLD_NFT_TOKEN_ID} | xxd -p -u | tr -d '\n')"

NEW_NFT_TOKEN_ID="CITYNFT-150731"
NEW_NFT_TOKEN_ID_HEX="0x$(echo -n ${NEW_NFT_TOKEN_ID} | xxd -p -u | tr -d '\n')"
NEW_NFT_TOKEN_ID_ONLY_HEX="$(echo -n ${NEW_NFT_TOKEN_ID} | xxd -p -u | tr -d '\n')"

CALLER_ADDRESS="erd149axj8feledcw7zck5f3ecwrncgd0gemcr9q69yxqlk0zvnl5zvs065jqu"
CALLER_ADDRESS_HEX="0x$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"
CALLER_ADDRESS_ONLY_HEX="$(erdpy wallet bech32 --decode ${CALLER_ADDRESS})"

################################################
ADDRESS=$(erdpy data load --key=address-devnet)
################################################

deploy() {
    erdpy --verbose contract deploy \
    --project=${PROJECT} \
    --recall-nonce \
    --pem=${WALLET} \
    --gas-limit=60000000 \
    --arguments ${OLD_NFT_TOKEN_ID_HEX} ${NEW_NFT_TOKEN_ID_HEX} \
    --send \
    --metadata-payable \
    --outfile="deploy-devnet.interaction.json" \
    --proxy=${PROXY} \
    --chain=${CHAIN_ID} || return

    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value=${ADDRESS}

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

UPGRADE_ADDRESS="erd1qqqqqqqqqqqqqpgqzrzd5c79y85jj56vf2nura3j4e96lpxd5zvsyq6lh7"
UPGRADE_ADDRESS_ONLY_HEX="$(erdpy wallet bech32 --decode ${UPGRADE_ADDRESS})"

upgrade() {
    erdpy --verbose contract upgrade ${UPGRADE_ADDRESS_ONLY_HEX} --project=${PROJECT} --recall-nonce --pem=${WALLET} --send --outfile="upgrade.json" --proxy=${PROXY} --chain=${CHAIN_ID} \
    --metadata-payable \
    --gas-limit=100000000 \
    --arguments ${OLD_NFT_TOKEN_ID_HEX} ${NEW_NFT_TOKEN_ID_HEX}
}
