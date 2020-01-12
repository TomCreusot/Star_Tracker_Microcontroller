DIR_SRC = src/
all: DToTree.jar DPreprocessor.jar



DToTree.jar: $(DIR_SRC)DToTree.jar $(DIR_SRC)Star.java $(DIR_SRC)CSV.java
	javac $(DIR_SRC)*.java
	cd $(DIR_SRC); jar cvfe $@ DToTree.jar DToTree.class Star.class CSV.class
	mv $(DIR_SRC)$@ .
	rm -f $(DIR_SRC)*.class



DPreprocessor.jar: $(DIR_SRC)DPreprocessor.java $(DIR_SRC)Star.java $(DIR_SRC)CSV.java
	javac $(DIR_SRC)*.java
	cd $(DIR_SRC); jar cvfe $@ DPreprocessor DPreprocessor.class Star.class CSV.class
	mv $(DIR_SRC)$@ .
	rm -f $(DIR_SRC)*.class


clean:
	rm -f $(DIR_SRC)*.class *.jar
