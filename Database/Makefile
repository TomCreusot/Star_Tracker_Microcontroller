DIR_SRC = src/
DIR_TEST = test/



all: DToTree.jar DPreprocessor.jar
test: $(DIR_TEST)TestHarness.class


DToTree.jar: $(DIR_SRC)DToTree.java $(DIR_SRC)Star.java $(DIR_SRC)FileIO.java $(DIR_SRC)Tree.java
	javac $(DIR_SRC)*.java -Xlint:unchecked
	cd $(DIR_SRC); jar cvfe $@ DToTree.jar DToTree.class Star.class FileIO.class Tree.class
	mv $(DIR_SRC)$@ .
	rm -f $(DIR_SRC)*.class


DPreprocessor.jar: $(DIR_SRC)DPreprocessor.java $(DIR_SRC)Star.java $(DIR_SRC)FileIO.java $(DIR_SRC)Tree.java
	javac $(DIR_SRC)*.java -Xlint:unchecked
	cd $(DIR_SRC); jar cvfe $@ DPreprocessor DPreprocessor.class Star.class FileIO.class LoadingBar.class Tree.class TreeNode.class
	mv $(DIR_SRC)$@ .
	rm -f $(DIR_SRC)*.class



$(DIR_TEST)TestHarness.class : $(DIR_SRC)* $(DIR_TEST)
	javac $(DIR_SRC)*.java $(DIR_TEST)*.java -Xlint:unchecked
	cp $(DIR_SRC)*.class $(DIR_TEST)
	cd $(DIR_TEST); java TestHarness
	rm -f $(DIR_TEST)*.class






clean:
	rm -f $(DIR_SRC)*.class *.jar
	rm -f $(DIR_TEST)*.class
	rm -f *.jar




install:
	-mkdir csv/
	wget -P csv/ http://www.astronexus.com/files/downloads/hygdata_v3.csv.gz
	gzip -d csv/hygdata_v3.csv.gz

uninstall:
	-rm -r -f csv/*
