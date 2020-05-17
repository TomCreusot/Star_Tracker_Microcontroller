fBold="\e[1mBold"
fRed="\033[0;31m"
fBRed="\e[41m"
fBDGrey="\e[100m"

OUT_DIR = out
LCOV_DIR = coverage
DOCS_DIR = documentation
LIBS_DIR = libs


run:
	bazel build //libs:demo
	@- rm -f /$(OUT_DIR)/demo
	mv -f bazel-bin/libs/demo ./$(OUT_DIR)/
	cd out; ./demo config.properties


database:
	bazel build //libs:database_generator
	@- rm -f /$(OUT_DIR)/database_generator
	mv -f bazel-bin/libs/database_generator ./$(OUT_DIR)/
	cd $(OUT_DIR); ./database_generator config.properties


docs: $(find $(DOCS_DIR) -type f) $(find $(LIBS_DIR) -type f)
	rm $(DOCS_DIR)/latex/ -f -r
	rm $(DOCS_DIR)/html/ -f -r
	doxygen $(DOCS_DIR)/doxygen_config

test:
	@ echo $(fBold) $(fBRed)		util				$(fNorm)
	bazel test //libs/util/... --test_output=all


	@ echo $(fBold) $(fBRed)		star_tracker		$(fNorm)
	# bazel test //libs/star_tracker/... --test_output=all


	@ echo $(fBold) $(fBRed)		image_processing	$(fNorm)
	bazel test //libs/image_processing/... --test_output=all

	@ echo $(fBold) $(fBRed)		nix			$(fNorm)
	bazel test //libs/nix/... --test_output=all


	# @ echo $(fBold) $(fBRed)		get_image			$(fNorm)
	# @ bazel test //libs/get_image/... --test_output=all


	# @ echo $(fBold) $(fBRed)		coms				$(fNorm)


lcov:
	@echo $(fBold) $(fBRed)\n		util		echo $(fNorm)
	bazel coverage //libs/util/... --combined_report=lcov
	lcov --l bazel-out/_coverage/_coverage_report.dat
	genhtml bazel-out/_coverage/_coverage_report.dat -o $(LCOV_DIR)/util -f

	@echo $(fBold) $(fBRed)\n		star_tracker		$(fNorm)
	bazel coverage //libs/star_tracker/... --combined_report=lcov
	lcov --l bazel-out/_coverage/_coverage_report.dat
	genhtml bazel-out/_coverage/_coverage_report.dat -o $(LCOV_DIR)/star_tracker -f

	@echo $(fBold) $(fBRed)\n		image_processing	$(fNorm)
	bazel coverage //libs/image_processing/... --combined_report=lcov
	lcov --l bazel-out/_coverage/_coverage_report.dat
	genhtml bazel-out/_coverage/_coverage_report.dat -o $(LCOV_DIR)/image_processing -f

	@echo $(fBold) $(fBRed)\n		nix					$(fNorm)
	bazel coverage //libs/nix/... --combined_report=lcov
	lcov --l bazel-out/_coverage/_coverage_report.dat
	genhtml bazel-out/_coverage/_coverage_report.dat -o $(LCOV_DIR)/nix -f


	@echo $(fBold) $(fBRed)\n		get_image					$(fNorm)
	bazel coverage //libs/get_image/... --combined_report=lcov
	lcov --l bazel-out/_coverage/_coverage_report.dat
	genhtml bazel-out/_coverage/_coverage_report.dat -o $(LCOV_DIR)/get_image -f



help:
	@echo Type 'make' to run a demo.
	@echo Type 'make database' to construct the database to use.
	@echo Type 'make docs' to generate doxygen documentation.
	@echo Type 'make test' to run a bazel test harness.
	@echo Type 'make lcov' to run test coverage.
	@echo Type 'make help' if you have no idea what you are doing, it wont help.
