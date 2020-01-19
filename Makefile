# This make file calls all other make files.


all:
	cd Star\ Tracker; make
	cd Database; make


clean:
	cd Star\ Tracker; make clean
	cd Database; make clean


install:
	cd Star\ Tracker; make install
	cd Database; make install

uninstall:
	cd Star\ Tracker; make uninstall
	cd Database; make uninstall






test:
	cd Database; make test
	cd Star\ Tracker; make test
