package(default_visibility = ["//visibility:public"])


cc_library(
	name="database_array",
	hdrs=["database_array.h"],
	srcs=["database_array.h"],
	deps=["//libs/util:util", ":database"]
)


cc_library(
	name="database",
	hdrs=["database.h"],
	deps=["//libs/util:util", ":star_set", "//libs/util:array_list", "//libs/util:point"]
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

cc_test(
	name = "database_test",
	srcs = ["database_test.cc"],
	deps = ["@gtest//:gtest_main", ":database", ":star_set", "//libs/util:array_list", "//libs/util:array_list_mock"]
)
