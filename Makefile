.PHONY: expand
expand:
	cargo expand --example=expand_me

.PHONY: build
build:
	cargo build

.PHONY: build-release
build-release:
	cargo lint && cargo build --release

.PHONY: clippy
clippy:
	cargo clippy --workspace --all-targets --all-features --examples --tests

.PHONY: check-all
check-all:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets --all-features --examples --tests
	cargo rustdoc --all-features -- -D warnings
	cargo test --workspace --all-features

.PHONY: doc
doc:
	cargo doc --all-features --open &

.PHONY: test
test:
	cargo test

.PHONY: test-and-publish
test-and-publish:
	cargo test --all-features && cargo publish

.PHONY: upgrade-check
upgrade-check:
	cargo upgrade --workspace --dry-run

.PHONY: todo
todo:
	rg 'TODO|FIXME|todo!' crates examples src tests
