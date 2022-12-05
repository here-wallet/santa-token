
build:
	cd contract && cargo build --target wasm32-unknown-unknown --release && cd ../ && \
	cp contract/target/wasm32-unknown-unknown/release/santa_token.wasm ./out/main.wasm

deploy-prod:
	make build && \
	NEAR_ENV=mainnet near deploy santa_token.near
