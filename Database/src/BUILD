package(default_visibility = ["//visibility:public"])

java_binary(
	name = "MainBinary",
	srcs = glob(["src/*.java"]),
	main_class = "Main"
)


java_library(
	name = "Main",
	srcs = ["Main.java"],
	deps = [":Preprocessor", ":FillTemplate", ":Point", ":StarSet", ":Star", ":FileIO"]
)



java_library(
	name = "Preprocessor",
	srcs = ["Preprocessor.java"],
	deps = [":Star", ":StarSet", ":FileIO"]
)


java_library(
	name = "FillTemplate",
	srcs = ["FillTemplate.java"],
	deps = [":Star", ":FileIO"]
)


java_library(
	name = "Star",
	srcs = ["Star.java"],
	deps = [":Point"]
)


java_library(
	name = "StarSet",
	srcs = ["StarSet.java"],
	deps = [":Point", ":Star"]
)


java_library(
	name = "Point",
	srcs = ["Point.java"],
)


java_library(
	name = "FileIO",
	srcs = ["FileIO.java"],
)
