package(default_visibility = ["//visibility:public"])

cc_library(
	name = "star_tracker",
	srcs = ["star_tracker.cc"],
	hdrs = ["star_tracker.h"],
	deps = [
			":combo",
			"//libs/util:point",
			"//libs/util:util",
			"//libs/image_processing:blob",
			"//libs/database:angle_stat"
			]
)

cc_library(
	name = "combo",
	srcs = ["combo.cc"],
	hdrs = ["star_tracker.h"],
	deps = [
			"//libs/util:point",
			"//libs/util:util",
			"//libs/image_processing:blob",
			"//libs/database:angle_stat"
			]
)






cc_test(
	name = "star_tracker_test",
	srcs = ["star_tracker_test.cc"],
	deps = [":star_tracker", ":combo", "//libs/image_processing:blob_mock", "@gtest//:gtest_main"]
)


cc_test(
	name = "combo_test",
	srcs = ["combo_test.cc"],
	deps = [":combo", "@gtest//:gtest_main"]
)
