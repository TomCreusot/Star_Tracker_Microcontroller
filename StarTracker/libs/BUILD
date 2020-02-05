cc_binary(
	name = "IP_Test",
	srcs = ["IP_Test.cpp"],
	deps =
		[
			"//libs/Utils:Utils",
			"//libs/Utils:Point",
			# "//libs/Utils:Image",

			":SimpleBMP",
			"//libs/ImageAcquisition:GetImage",

			"//libs/ImageProcessing:ImageProcessing",
			"//libs/ImageProcessing:Blob",

			"//libs/StarTracker:StarTracker"
		]
)




config_settings(
	name = "embedded",
	values = {"compilation_mode" : "embedded"}
)




config_settings(
	name = "nix",
	values = {"system" : "unix"}
)
