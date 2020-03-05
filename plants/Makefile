all:
	cargo build

test1: all
	./target/debug/plants F-F-F-F ../tests/rules1.txt 3 > result.txt
	python3 ../graphics/src/2d.py result.txt
