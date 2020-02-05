cc_library(
	name = "get_image",
	srcs = ["get_image.cpp"],
	hdrs = ["get_image.hpp"],
	deps = ["//libs/get_image:get_image_interface", "@net_sourceforge_easybmp//:EasyBMP", "//libs/image_processing:image_interface"],
)


cc_test(
	name = "get_image_test",
	srcs = ["get_image_test.cc"],
	deps = ["@gtest//:gtest_main", ":get_image"],

)
