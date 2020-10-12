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


			"//config:demo",
			"//config:runtime",
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
			"//libs/nix:star",
			"//libs/nix:fill_template",
			"//libs/nix:linked_list",

			"//libs/star_tracker:star_set",
			"//libs/star_tracker:database",


			"//config:database",
		]
)
