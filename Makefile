.PHONY: test

dev:
	cd ./sdk
    IDNS_ETH_PATH=../target cargo run --example account