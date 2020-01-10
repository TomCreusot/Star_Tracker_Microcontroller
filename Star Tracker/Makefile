# $@: rule, $^ prerecs

CC = g++
FLAGS = -Wall -g -O3
EXEC = Processor


DIR = src/
DIR_IM = $(DIR)ImageProcessing/
DIR_BMP = $(DIR)EasyBMP/
DIR_OBJ = obj_files/

FILES = $(DIR_OBJ)Main.o $(DIR_OBJ)ImageProcessing.o $(DIR_OBJ)Blob.o $(DIR_OBJ)EasyBMP.o


#default: $(EXEC)

ifndef TEST

#main
$(EXEC): $(FILES)
	$(CC) $(FLAGS) -o $@ $^
	-./$(EXEC)

$(DIR_OBJ)Main.o: $(DIR)Main.cpp $(DIR_OBJ)EasyBMP.o $(DIR_OBJ)ImageProcessing.o $(DIR_OBJ)Blob.o
	$(CC) $(FLAGS) -c $(DIR)Main.cpp
	mv main.o $(DIR_OBJ)

endif


$(DIR_OBJ)ImageProcessing.o: $(DIR_IM)ImageProcessing.hpp $(DIR_IM)ImageProcessing.cpp $(DIR_OBJ)Blob.o
	$(CC) $(FLAGS) -c $(DIR_IM)ImageProcessing.cpp
	mv ImageProcessing.o $(DIR_OBJ)


$(DIR_OBJ)Blob.o: $(DIR_IM)ImageProcessing.hpp $(DIR_IM)Blob.cpp
	$(CC) $(FLAGS) -c $(DIR_IM)Blob.cpp
	mv Blob.o $(DIR_OBJ)




# Remove for final build (test code).
$(DIR_OBJ)EasyBMP.o: $(DIR_BMP)EasyBMP.cpp $(DIR_BMP)EasyBMP*.h
	$(CC) $(CFLAGS) -c $(DIR_BMP)EasyBMP.cpp
	mv EasyBMP.o $(DIR_OBJ)




clean:
	-rm -f $(FILES)
