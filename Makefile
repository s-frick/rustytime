
build:
	@cargo build

watch: 
	@cargo watch -c -w src -x build
