package(default_visibility = ["//visibility:public"])

cc_library(
	name = "point",
	srcs = ["point.h"]
)



cc_library(
	name = "util",
	hdrs = ["util.h"],
	srcs = ["util.h"],
)


cc_library(
	name = "array_list",
	hdrs = ["array_list.h"],
	srcs = ["array_list.h"]
)


cc_library(
	name = "array_list_mock",
	srcs = ["array_list_mock.h", "array_list.h"]
)



cc_test(
	name = "point_test",
	srcs = ["point_test.cc"],
	deps = [":point", ":util", "@gtest//:gtest_main"]
)


cc_test(
	name = "array_list_test",
	srcs = ["array_list_test.cc"],
	deps = [":array_list", "@gtest//:gtest_main"]
)




cc_test(
	name = "array_list_mock_test",
	srcs = ["array_list_mock_test.cc"],
	deps = [":array_list_mock", "@gtest//:gtest_main"]
)
