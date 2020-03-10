all:
	cargo build

test1:
	./target/debug/plants F-F-F-F ../tests/rules1.txt 3
	python3 ../graphics/src/2d.py result.txt 90

test2:
	./target/debug/plants F+F+F+F ../tests/rules2.txt 2
	python3 ../graphics/src/2d.py result.txt 22.5

test3:
	./target/debug/plants FX ../tests/rules3.txt 10
	python3 ../graphics/src/2d.py result.txt 22.5

test4:
	./target/debug/plants FFF ../tests/rules4.txt 10
	python3 ../graphics/src/2d.py result.txt 22.5
