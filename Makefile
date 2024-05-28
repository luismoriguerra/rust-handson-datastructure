


run:
	cargo watch -q -c -w src/ -x run

test:
	cargo watch -q -c -w tests/ -x "test -q quick_dev -- --nocapture"

example:
	cargo run -q examples $(n)

we:
	cargo watch -q -c -w examples/ -x "build --release"

# function cwe() {
#   cargo watch -q -c -x "run -q --example $1"
# }

# function cre() {
#   cargo run --example $1
# }