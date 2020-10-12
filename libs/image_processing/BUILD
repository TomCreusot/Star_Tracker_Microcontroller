package(default_visibility = ["//visibility:public"])


cc_library(
	name = "blob",
	srcs = ["blob.cc"],
	hdrs = ["blob.h"],
	deps = [
				"//libs/util:point",
				":image",
				"//libs/util:util",
				"//libs/util:array_list",
			]
)


cc_test(
	name = "implementation_test",
	srcs = ["implementation_test.cc"],
	deps = ["//libs/util:util", "//libs/util:array_list", "//libs/util:point", ":image", ":blob", "@gtest//:gtest_main"]
	)






cc_library(
	name = "image",
	hdrs = ["image.h"],
	srcs = ["image.cc"],
	deps = ["//libs/util:util", "//libs/util:array_list", "//config:runtime"]
)


cc_library(
	name = "blob_mock",
	srcs = ["blob_mock.h"],
	deps = ["//libs/image_processing:blob"]
	)



cc_test(
	name = "image_test",
	srcs = ["image_test.cc"],
	deps = ["@gtest//:gtest_main", ":image", "//config:runtime"],
)


cc_test(
	name = "blob_test",
	srcs = ["blob_test.cc"],
	deps = [":blob", "@gtest//:gtest_main"]
)
