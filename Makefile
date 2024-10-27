RUST_DIR=rust
PYO3_DIR=$(RUST_DIR)/configler-pyo3

HOST_ARCH = $(shell rustc -vV | grep '^host:' | cut -d' ' -f2)

help: ## Print the help documentation
	@grep -E '^[/a-zA-Z0-9_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'


#######################################################
######## Project Build Operations #####################
#######################################################

build-rust: ## Build rust project
	(cd $(RUST_DIR); cargo build)

run-rust:  ## run rust project, temporary
	(cd $(RUST_DIR); cargo run)


install-llvm-cov: ## installs rust code coverage tool
	curl --proto '=https' --tlsv1.2 -fsSL https://github.com/taiki-e/cargo-llvm-cov/releases/latest/download/cargo-llvm-cov-$(HOST_ARCH).tar.gz | tar xzf - -C "$(HOME)/.cargo/bin"

#######################################################
######## Project Code Quality Operations###############
#######################################################

format-rust: ## Format rust code
	(cd $(RUST_DIR); cargo fmt)

lint-rust:  ## Lint rust code
	(cd $(RUST_DIR); cargo clippy)

test-coverage-rust: ## Test rust code & verify test coverage
	(cd $(RUST_DIR); cargo llvm-cov --all-features --workspace  --html)
	(cd $(RUST_DIR); cargo llvm-cov report --fail-under-lines 90)

audit-rust:  ## Audit rust dependencies for vulnerabilities
	(cd $(RUST_DIR); cargo audit)


#######################################################
######## Language Specific Operations   ###############
#######################################################

install-python-deps: ## Install python dependencies for configler-pyo3
	(cd $(PYO3_DIR); python3 -m venv .venv; . .venv/bin/activate; pip3 install -r requirements.txt)

build-python-bindings: ## Build & install rust bindings for python
	(cd $(PYO3_DIR); . .venv/bin/activate; maturin develop)

start-python-shell: build-python-bindings ## Start python shell with python bindings
	(cd $(PYO3_DIR); . .venv/bin/activate; python3)