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


#######################################################
######## Language Specific Operations   ###############
#######################################################

install-python-deps: ## Install python dependencies for configler-pyo3
	(cd rust/configler-pyo3; python3 -m venv .venv; . .venv/bin/activate; pip3 install -r requirements.txt)

build-python-bindings: ## Build & install rust bindings for python
	(cd rust/configler-pyo3; . .venv/bin/activate; maturin develop)

start-python-shell: build-python-bindings ## Start python shell with python bindings
	(cd rust/configler-pyo3; . .venv/bin/activate; python3)