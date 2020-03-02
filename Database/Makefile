FOV=10
MAX_MAG=6
NUM_A_PILOT=10 #stars around pilot

all:
	javac libs/*.java
	cd libs; java Preprocessor ../resources/hip_mag_ra_dec.csv ../resources/out.csv $(FOV) $(MAX_MAG) $(NUM_A_PILOT)
	cd libs; java FillTemplate ../resources/template.cc ../resources/out.csv ../resources/database_array.h database_array
	cp -r resources/database_array.h ../StarTracker/libs/database
	# g++ resources/database_array.cc -c
	rm -f resources/out.csv
