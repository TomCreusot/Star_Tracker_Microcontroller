package(default_visibility = ["//visibility:public"])



cc_library(
	name="database_array",
	hdrs=["database_array.h"],
	deps=["//libs/util:util"]
)




cc_library(
	name = "star_set",
	hdrs = ["star_set.h"],
	srcs = ["star_set.cc"],
	deps = ["//libs/util:point", "//libs/util:util", "//libs/util:array_list"]
)



cc_test(
	name = "star_set_test",
	srcs = ["star_set_test.cc"],
	deps = ["@gtest//:gtest_main", ":star_set", "//libs/util:array_list", "//libs/util:array_list_mock"]
)
