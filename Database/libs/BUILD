java_binary(
	name = "Preprocessor",
	srcs = ["Preprocessor.java"],
	deps = [":Star", ":FileIO"]
)

java_binary(
	name = "FillTemplate",
	srcs = ["FillTemplate.java"],
	deps = [":Star", ":FileIO"]
)





java_library(
	name = "Star",
	srcs = ["Star.java"],
)


java_library(
	name = "FileIO",
	srcs = ["FileIO.java"],
)



java_test(
	name = "FileIOTest",
	srcs = ["FileIOTest.java"],
	deps = [":FileIO"]
)
