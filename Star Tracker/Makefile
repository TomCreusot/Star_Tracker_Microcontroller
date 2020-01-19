# $@: rule, $^ prerecs

CC = g++
FLAGS = -Wall -g -O3
EXEC_IP = IP_EXAMPLE.out
EXEC_DB = DB_EXAMPLE.out

EXEC_TEST = TEST_HARNESS.out

DIR = src/
DIR_IP = $(DIR)ImageProcessing/
DIR_ST = $(DIR)StarTracker/
DIR_DB = $(DIR)Database/
DIR_BMP = $(DIR)EasyBMP/
DIR_OBJ = obj_files/
DIR_DEMO = demos/

DIR_TEST = test/

ifdef COMP
IP_FLAG = -D COMPUTER=1
else
IP_FLAG =
endif


FILES = $(DIR_OBJ)ImageProcessing.o $(DIR_OBJ)Blob.o $(DIR_OBJ)Point.o $(DIR_OBJ)EasyBMP.o $(DIR_OBJ)StarTracker.o $(DIR_OBJ)Database.o

TEST_FILES = $(DIR_OBJ)StarTracker.Test.o $(DIR_OBJ)ImageProcessing.Test.o $(DIR_OBJ)UnitTest.o

#default: $(EXEC)

all: $(EXEC_IP) $(EXEC_DB)
test: $(EXEC_TEST)

#main
$(EXEC_IP): $(FILES) $(DIR_OBJ)IP_Main.o
	$(CC) $(FLAGS) -o $@ $^

$(EXEC_DB): $(FILES) $(DIR_OBJ)DB_Main.o
	$(CC) $(FLAGS) -o $@ $^

$(EXEC_TEST): $(TEST_FILES) $(FILES) $(DIR_OBJ)TestHarness.o
	$(CC) $(FLAGS) -o $@ $^
	./$@





$(DIR_OBJ)DB_Main.o: $(FILES) $(DIR_DEMO)DB_Main.cpp
	$(CC) $(FLAGS) -c $(DIR_DEMO)DB_Main.cpp
	mv DB_Main.o $(DIR_OBJ)


$(DIR_OBJ)IP_Main.o: $(FILES) $(DIR_DEMO)IP_Main.cpp
	$(CC) $(FLAGS) -c $(DIR_DEMO)IP_Main.cpp
	mv IP_Main.o $(DIR_OBJ)


$(DIR_OBJ)TestHarness.o: $(TEST_FILES) $(FILES) $(DIR_TEST)TestHarness.cpp
	$(CC) $(FLAGS) -c $(DIR_TEST)TestHarness.cpp
	mv TestHarness.o $(DIR_OBJ)





$(DIR_OBJ)StarTracker.o: $(DIR_ST)StarTracker.hpp $(DIR_IP)ImageProcessing.cpp $(DIR_DB)Database.cpp $(DIR_ST)StarTracker.cpp
	$(CC) $(FLAGS) -c $(DIR_ST)StarTracker.cpp
	mv StarTracker.o $(DIR_OBJ)


$(DIR_OBJ)Database.o: $(DIR_DB)Database.hpp $(DIR_IP)ImageProcessing.cpp $(DIR_IP)ImageProcessing.hpp $(DIR_IP)Point.cpp $(DIR_DB)Database.cpp
	$(CC) $(FLAGS) -c $(DIR_DB)Database.cpp
	mv Database.o $(DIR_OBJ)





$(DIR_OBJ)ImageProcessing.o: $(DIR_IP)ImageProcessing.hpp $(DIR_IP)Blob.cpp $(DIR_IP)Point.cpp $(DIR_IP)ImageProcessing.cpp
	$(CC) $(FLAGS) -c $(DIR_IP)ImageProcessing.cpp $(IP_FLAG)
	mv ImageProcessing.o $(DIR_OBJ)


$(DIR_OBJ)Blob.o: $(DIR_IP)ImageProcessing.hpp $(DIR_IP)Point.cpp $(DIR_IP)Blob.cpp
	$(CC) $(FLAGS) -c $(DIR_IP)Blob.cpp
	mv Blob.o $(DIR_OBJ)

$(DIR_OBJ)Point.o: $(DIR_IP)Point.cpp
	$(CC) $(FLAGS) -c $(DIR_IP)Point.cpp
	mv Point.o $(DIR_OBJ)




$(DIR_OBJ)EasyBMP.o: $(DIR_BMP)EasyBMP.cpp $(DIR_BMP)EasyBMP*.h
	$(CC) -c $(DIR_BMP)EasyBMP.cpp
	mv EasyBMP.o $(DIR_OBJ)









# Test files
$(DIR_OBJ)StarTracker.Test.o: $(DIR_TEST)StarTracker.Test.cpp $(DIR_TEST)UnitTest.hpp $(DIR_ST)StarTracker.cpp
	$(CC) $(FLAGS) -c $(DIR_TEST)StarTracker.Test.cpp
	mv StarTracker.Test.o $(DIR_OBJ)


$(DIR_OBJ)ImageProcessing.Test.o: $(DIR_TEST)ImageProcessing.Test.cpp $(DIR_TEST)UnitTest.hpp $(DIR_IP)ImageProcessing.cpp
	$(CC) $(FLAGS) -c $(DIR_TEST)ImageProcessing.Test.cpp
	mv ImageProcessing.Test.o $(DIR_OBJ)


$(DIR_OBJ)UnitTest.o: $(DIR_TEST)UnitTest.cpp
	$(CC) $(FLAGS) -c $(DIR_TEST)UnitTest.cpp
	mv UnitTest.o $(DIR_OBJ)










clean:
	-rm -f $(DIR_OBJ)*



install:
	-mkdir dependancies/
	-mkdir dependancies/EasyBMP/
	-mkdir $(DIR)/EasyBMP/
	wget -P dependancies/EasyBMP http://prdownloads.sourceforge.net/easybmp/EasyBMP_1.06.zip?download
	mv dependancies/EasyBMP/* dependancies/EasyBMP/EasyBMP.zip
	unzip dependancies/EasyBMP/EasyBMP.zip -d dependancies/EasyBMP/

	wget -P dependancies/ http://prdownloads.sourceforge.net/easybmp/EasyBMP_Documentation_1.06.00.zip?download

	cp dependancies/EasyBMP/EasyBMP_BMP.h $(DIR_BMP)
	cp dependancies/EasyBMP/EasyBMP_DataStructures.h $(DIR_BMP)
	cp dependancies/EasyBMP/EasyBMP_VariousBMPutilities.h $(DIR_BMP)
	cp dependancies/EasyBMP/EasyBMP.h $(DIR_BMP)
	cp dependancies/EasyBMP/EasyBMP.cpp $(DIR_BMP)



uninstall:
	-rm -r -f dependancies/
	-rm -r -f src/EasyBMP/
	-rm -f $(FILES)
