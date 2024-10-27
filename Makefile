RUST_DIR=rust
PYO3_DIR=$(RUST_DIR)/configler-pyo3

help: ## Print the help documentation
	@grep -E '^[/a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


#######################################################
######## Project Build Operations #####################
#######################################################

build-rust: ## Build rust project
	(cd $(RUST_DIR); cargo build)

run-rust:  ## run rust project, temporary
	(cd $(RUST_DIR); cargo run)

#######################################################
######## Project Code Quality Operations###############
#######################################################

format-core-rust:
	(cd $(RUST_DIR); cargo fmt)


#######################################################
######## Language Specific Operations   ###############
#######################################################

install-python-deps: ## Install python dependencies for configler-pyo3
	(cd $(PYO3_DIR); python3 -m venv .venv; . .venv/bin/activate; pip3 install -r requirements.txt)

build-python-bindings: ## Build & install rust bindings for python
	(cd $(PYO3_DIR); . .venv/bin/activate; maturin develop)

start-python-shell: build-python-bindings ## Start python shell with python bindings
	(cd $(PYO3_DIR); . .venv/bin/activate; python3)