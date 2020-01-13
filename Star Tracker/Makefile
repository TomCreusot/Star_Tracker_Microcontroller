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



install:
	-mkdir dependancies/
	-mkdir dependancies/EasyBMP/
	-mkdir $(DIR)/EasyBMP/
	git clone https://github.com/aburgh/EasyBMP --depth 1 --branch=master dependancies/EasyBMP

	cp dependancies/EasyBMP/EasyBMP_BMP.h $(DIR_BMP)
	cp dependancies/EasyBMP/EasyBMP_DataStructures.h $(DIR_BMP)
	cp dependancies/EasyBMP/EasyBMP_VariousBMPutilities.h $(DIR_BMP)
	cp dependancies/EasyBMP/EasyBMP.h $(DIR_BMP)
	cp dependancies/EasyBMP/EasyBMP.cpp $(DIR_BMP)
	cp dependancies/EasyBMP/readme.markdown $(DIR_BMP)
	cp dependancies/EasyBMP/EasyBMP_UserManual.pdf $(DIR_BMP)

uninstall:
	-rm -r -f dependancies/
	-rm -r -f src/EasyBMP/
	-rm -f $(FILES)
