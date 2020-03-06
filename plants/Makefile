all:
	cargo build

test1:
	./target/debug/plants F-F-F-F ../tests/rules1.txt 3 > result.txt
	python3 ../graphics/src/2d.py result.txt

test2:
	./target/debug/plants F+F+F+F ../tests/rules2.txt 7 > result.txt
	python3 ../graphics/src/2d.py result.txt

test3:
	./target/debug/plants FX ../tests/rules3.txt 10 > result.txt
	python3 ../graphics/src/2d.py result.txt

test4:
	./target/debug/plants FFF ../tests/rules4.txt 10 > result.txt
	python3 ../graphics/src/2d.py result.txt
