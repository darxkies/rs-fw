RUST_MIPS_MUSL_VERSION = 2020.02.3.1
RUST_MIPS_MUSL_IMAGE = darxkies/rust-mips-musl:$(RUST_MIPS_MUSL_VERSION)
RUST_MIPS_MUSL_CONTAINER = docker run -ti --rm -v $$(pwd):/workspace/project -v $$(pwd)/target/registry:/root/.cargo/registry $(RUST_MIPS_MUSL_IMAGE)

run:
	RUST_LOG=debug cargo run -- 

compile:
	cargo build

compile-mips:
	$(RUST_MIPS_MUSL_CONTAINER) cargo build --release

watch-and-compile:
	watchexec -e rs -r make run 
