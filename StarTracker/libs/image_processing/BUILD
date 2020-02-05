package(default_visibility = ["//visibility:public"])


cc_library(
	name = "blob",
	srcs = ["blob.cpp"],
	hdrs = ["blob.hpp"],
	deps = [
				"//libs/util:point",
				":image_base",
				"//libs/util:util",
				"//libs/util:array_list"
			]
)


cc_library(
	name = "image_interface",
	hdrs = ["image_interface.hpp"],
	deps = ["//libs/utils:utils"],
)

cc_library(
	name = "image_base",
	hdrs = ["image_base.hpp"],
	srcs = ["image_base.cpp"],
	deps = [":image_interface"]
)


cc_library(
	name = "blob_mock",
	srcs = ["blob_mock.hpp"],
	deps = ["//libs/image_processing:blob"]
	)



cc_test(
	name = "image_base_test",
	srcs = ["image_base_test.cc"],
	deps = [
	"@gtest//:gtest_main", ":image_base", "//libs/image_processing/impl/nix:image_impl"],
)


cc_test(
	name = "blob_test",
	srcs = ["blob_test.cc"],
	deps = [":blob", "//libs/image_processing/impl/nix:image_impl", "@gtest//:gtest_main"]
)
