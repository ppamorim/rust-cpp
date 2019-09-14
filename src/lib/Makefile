CC=gcc
CXX=g++
CFLAGS=-Wall

all: sum

sum: sum.o

sum.o: sum.cpp sum.h

sum.a: sum.o
	ar rcs libsum.a sum.o

libs: sum.a

clean:
	rm -f sum sum.o sum.a
run: sum
	./sum