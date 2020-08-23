fBold="\e[1mBold"
fRed="\033[0;31m"
fBRed="\e[41m"
fBDGrey="\e[100m"

OUT_DIR = out
LCOV_DIR = coverage
DOCS_DIR = documentation
LIBS_DIR = libs


run:
	clear
	reset
	bazel build //libs:runtime_properties_generator
	mv -f bazel-bin/libs/runtime_properties_generator ./$(OUT_DIR)/
	cd $(OUT_DIR); ./runtime_properties_generator config.properties
	@- rm -f /$(OUT_DIR)/runtime_properties_generator

	bazel build //libs:demo
	@- rm -f /$(OUT_DIR)/demo
	mv -f bazel-bin/libs/demo ./$(OUT_DIR)/
	cd $(OUT_DIR); ./demo config.properties

valgrind:
	clear
	reset
	bazel run //libs:demo --config debug --run_under='valgrind --tool=massif'
	# cd $(OUT_DIR); valgrind --leak-check=yes --track-origins=yes ./demo config.properties


database:
	clear
	reset
	bazel build //libs:database_generator
	@- rm -f /$(OUT_DIR)/database_generator
	mv -f bazel-bin/libs/database_generator ./$(OUT_DIR)/
	cd $(OUT_DIR); ./database_generator config.properties


docs: $(find $(DOCS_DIR) -type f) $(find $(LIBS_DIR) -type f)
	clear
	reset
	rm $(DOCS_DIR)/latex/ -f -r
	rm $(DOCS_DIR)/html/ -f -r
	sudo doxygen doxygen_config

test:
	clear
	reset
	@ echo $(fBold) $(fBRed)		util				$(fNorm)
	bazel test //libs/util/... --test_output=all


	@ echo $(fBold) $(fBRed)		star_tracker		$(fNorm)
	bazel test //libs/star_tracker/... --test_output=all


	@ echo $(fBold) $(fBRed)		image_processing	$(fNorm)
	bazel test //libs/image_processing/... --test_output=all

	@ echo $(fBold) $(fBRed)		nix			$(fNorm)
	bazel test //libs/nix/... --test_output=all


lcov:
	clear
	reset
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


help:
	@echo "This makefile is to help assist with long commands:"
	@echo "\t- Type 'make' to run a demo."
	@echo "\t- Type 'make database' to construct the database to use."
	@echo "\t- Type 'make docs' to generate doxygen documentation."
	@echo "\t- Type 'make test' to run a bazel test harness."
	@echo "\t- Type 'make lcov' to run test coverage."
	@echo "\t- Type 'make help' if you have no idea what you are doing, it wont help."


clean:
	clear
	reset
	bazel clean
	rm -f $(OUT_DIR)/demo
	rm -f $(OUT_DIR)/runtime_properties_generator
	rm -f $(OUT_DIR)/database_generator
	rm -f $(OUT_DIR)/out.bmp
	rm -r -f $(DOCS_DIR)
	rm -r -f $(LCOV_DIR)
	rm -r -f coverage
