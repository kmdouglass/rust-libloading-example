CC=gcc
CFLAGS=-Wall -Werror
OBJ=libffi-test.o

libffi-test.so: $(OBJ)
	gcc -shared -o $@ $(OBJ)

$(OBJ): ffi-test.c ffi-test.h
	$(CC) -c -o $(OBJ) -fpic $< $(CFLAGS)
