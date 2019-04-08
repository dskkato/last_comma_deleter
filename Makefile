.PHONY: all
all:
	cat test_data/sample.csv
	cargo run -- test_data/sample.csv test_data/output.csv
	cat test_data/output.csv
