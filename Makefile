EXPORT = export RUSTPATH=$(PWD)


checks:
	$(EXPORT) && cargo fmt
	$(EXPORT) && cargo clippy