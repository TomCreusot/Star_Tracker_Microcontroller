CC = g++
FLAGS = -Wall -g -O3
EXEC = Processor

DIR_OBJ = ../obj_files/


IP_EXAMPLE: $(DIR_OBJ)ImageProcessingExample.o
	-rm -f $(DIR_OBJ)Main.o
	$(CC) $(FLAGS) -o $@ $(DIR_OBJ)*

$(DIR_OBJ)ImageProcessingExample.o: ImageProcessingExample.cpp
	(cd ..;make TEST=1)
	$(CC) $(FLAGS) -c $<
	mv ImageProcessingExample.o $(DIR_OBJ)

clean:
	rm -f $(DIR_OBJ)ImageProcessingExample.o IP_EXAMPLE
