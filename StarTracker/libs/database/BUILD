package(default_visibility = ["//visibility:public"])

cc_library(
	name = "angle_stat",
	hdrs = ["angle_stat.h"],
	srcs = ["angle_stat.cc"],
	deps = ["//libs/util:point", "//libs/util:util", "//libs/util:array_list"]

)



cc_library(
	name = "database_array",
	hdrs = ["database_array.h"],
	deps = ["//libs/util:util"]
	)


cc_library(
	name = "find_elements",
	hdrs = ["find_elements.h"],
	srcs = ["find_elements.cc"],
	deps = ["//libs/util:util", ":database_array", "//libs/util:array_list", ":angle_stat"]
	)


cc_test(
	name = "angle_stat_test",
	srcs = ["angle_stat_test.cc"],
	deps = ["@gtest//:gtest_main", ":angle_stat"]
)
