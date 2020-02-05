package(default_visibility = ["//visibility:public"])
cc_library(
	name = "image_impl",
	hdrs = ["image_impl.hpp"],
	srcs = ["image_impl.cpp"],

	deps = ["//libs/image_processing:image_base"],
	strip_include_prefix = ".",
)


cc_test(
	name = "image_impl_test",
	srcs = ["image_impl_test.cc"],

	deps = ["@gtest//:gtest_main", ":image_impl"]
)
