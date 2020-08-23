cc_binary(
	name = "demo",
	srcs = ["demo.cc"],
	deps =
		[
			"//libs/util:util",
			"//libs/util:point",
			"//libs/util:array_list",


			"//libs/image_processing:image",
			"//libs/image_processing:blob",

			"//libs/star_tracker:star_set",
			"//libs/star_tracker:database_array",

			"//libs/nix:get_image",
			"//libs/nix:config",

			"//libs/properties:properties"
		]
)



cc_binary(
	name = "database_generator",
	srcs = ["database_generator.cc"],
	deps =
		[
			"//libs/util:util",
			"//libs/util:point",
			"//libs/util:array_list",
			"//libs/nix:config",
			"//libs/nix:star",
			"//libs/nix:fill_template",
			"//libs/nix:linked_list",

			"//libs/star_tracker:star_set",
			"//libs/star_tracker:database",
		]
)



cc_binary(
	name = "runtime_properties_generator",
	srcs = ["runtime_properties_generator.cc"],
	deps =
		[
			"//libs/util:util",
			"//libs/nix:config",
			"//libs/nix:fill_template",
		]
)
