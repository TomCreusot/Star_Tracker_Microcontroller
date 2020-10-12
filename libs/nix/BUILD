package(default_visibility = ["//visibility:public"])

cc_library(
	name = "star",
	hdrs = ["star.h"],
	srcs = ["star.h"],
	deps = ["//libs/util:array_list", "//libs/util:point", "//libs/util:util"]
)

cc_test(
	name = "star_test",
	srcs = ["star_test.cc"],
	deps = ["@gtest//:gtest_main", ":star", "//libs/util:array_list"]
)




cc_library(
	name = "fill_template",
	hdrs = ["fill_template.h"],
	srcs = ["fill_template.h"],
	deps = []
)

cc_test(
	name = "fill_template_test",
	srcs = ["fill_template_test.cc"],
	deps = ["@gtest//:gtest_main", ":fill_template"]
)




cc_library(
	name = "linked_list",
	hdrs = ["linked_list.h"],
	srcs = ["linked_list.h"],
	deps = ["//libs/util:util", "//libs/util:array_list"]
)

cc_test(
	name = "linked_list_test",
	srcs = ["linked_list_test.cc"],
	deps = ["@gtest//:gtest_main", ":linked_list", "//libs/util:point"]
)




cc_library(
	name = "get_image",
	srcs = ["get_image.cc"],
	hdrs = ["get_image.h"],
	deps = ["@net_sourceforge_easybmp//:EasyBMP", "//libs/image_processing:image", "//libs/util:array_list", "//libs/util:util", "//libs/util:point"],
)
