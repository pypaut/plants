all:
	cargo build

test1:
	./target/debug/plants ../tests/rules1.txt result.txt
	python3 ../graphics/src/2d.py result.txt 90

test2:
	./target/debug/plants ../tests/rules2.txt result.txt
	python3 ../graphics/src/2d.py result.txt 90

test3:
	./target/debug/plants ../tests/rules3.txt result.txt
	python3 ../graphics/src/2d.py result.txt 22.5

test4:
	./target/debug/plants ../tests/rules4.txt result.txt
	python3 ../graphics/src/2d.py result.txt 22.5

test5:
	./target/debug/plants ../tests/rules5.txt result.txt
	../graphics/graph3d/target/debug/graph3d result.txt result 22.5 10 0.5 0.1 5

test6:
	./target/debug/plants ../tests/rules6.txt result.txt
	../graphics/graph3d/target/debug/graph3d result.txt result 22.5 10 0.5 0.1 5

test7:
	./target/debug/plants ../tests/rules7.txt result.txt
	../graphics/graph3d/target/debug/graph3d result.txt result 22.5 10 0.5 0.1 5

test8:
	./target/debug/plants ../tests/rules8.txt result.txt
	python3 ../graphics/src/2d.py result.txt 22.5
