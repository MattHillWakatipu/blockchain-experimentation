# TBD

### Set variable for contract address
source neardev/dev-account.env

### Initialise newly deployed contract
near call $CONTRACT_NAME new_default_meta '{"owner_id": "'$CONTRACT_NAME'"}' --accountId $CONTRACT_NAME

### Mint an NFT
near call $CONTRACT_NAME nft_mint '{"token_id": "0", "receiver_id": "'isparx.testnet'", "metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", "copies": 1}}' --accountId $CONTRACT_NAME --deposit 10

near call $con nft_mint '{"token_id": "0", "receiver_id": "'isparx.testnet'", "metadata": { "title": "Olympus Mons", "description": "Tallest mountain in charted solar system", "media": "https://upload.wikimedia.org/wikipedia/commons/thumb/0/00/Olympus_Mons_alt.jpg/1024px-Olympus_Mons_alt.jpg", "copies": 1}}' --accountId $con --deposit 1
