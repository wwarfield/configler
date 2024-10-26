help: ## Print the help documentation
	@grep -E '^[/a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


#######################################################
######## Project Build Operations #####################
#######################################################

build-core-rust:
	(cd core-rust; cargo build)

run-core-rust:
	(cd core-rust; cargo run)

#######################################################
######## Project Code Quality Operations###############
#######################################################

format-core-rust:
	(cd core-rust; cargo fmt)