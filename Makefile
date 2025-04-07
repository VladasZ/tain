
lint:
	./scripts/lint.sh

test:
	cargo test --all && cargo test --all --release

tag:
	git tag v0.6.1
	git push origin v0.6.1 --force
