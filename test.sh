NEAR_ENV=mainnet near call santa_token.near ft_transfer '{"amount":"100", "receiver_id":"mydev.near"}' --gas 242794783120800 --accountId santa.herewallet.near --depositYocto 1
NEAR_ENV=mainnet near view santa_token.near get_seed ''
