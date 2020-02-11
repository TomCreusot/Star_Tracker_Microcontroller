FOV=60
MAX_MAG=4
NUM_A_PILOT=20 #stars around pilot

all:
	javac libs/*.java
	# cd libs; java Preprocessor ../resources/hip_mag_ra_dec.csv ../resources/out.csv $(FOV) $(MAX_MAG) $(NUM_A_PILOT)
	cd libs; java FillTemplate ../resources/c_template.txt ../resources/out.csv ../resources/database_array.cc array
	g++ resources/database_array.cc -c
	rm -f resources/out.csv
