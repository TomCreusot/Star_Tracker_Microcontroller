dir=//libs/image_processing
coverage_dir=bazel-out/k8-fastbuild/testlogs$(dir)

fNorm="\e[0m"
fBold="\e[1mBold"
fRed="\033[0;31m"
fBRed="\e[41m"
fBDGrey="\e[100m"

testToRun=//libs/image_processing:image_base_test#impl/embedded:image_impl_test

all:
	@echo image_base_test.cc
	bazel test $(dir):image_base_test --test_output=all
	@echo "\n\n\n\n\n"



	@echo impl/nix/image_impl_test.cc
	bazel test $(dir)/impl/nix:image_impl_test --test_output=all
	@echo "\n\n\n\n\n"

	@echo impl/embedded/image_impl_test.cc
	bazel test $(dir)impl/embedded:image_impl_test --test_output=all
	@echo "\n\n\n\n\n"



	@echo blob_test.cc
	bazel test $(dir):blob_test --test_output=all
	@echo "\n\n\n\n\n"






	bazel coverage $(dir):image_base_test
	bazel coverage $(dir)/impl/nix:image_impl_test
	bazel coverage $(dir)impl/embedded:image_impl_test
	bazel coverage $(dir):image_base_test
	@echo $(fBDGrey)
	@lcov --summary -l $(coverage_dir)/image_base_test/coverage.dat
	@lcov --summary -l $(coverage_dir)/blob_test/coverage.dat
	@lcov --summary -l $(coverage_dir)/impl/nix/image_impl_test/coverage.dat
	@lcov --summary -l $(coverage_dir)/impl/embedded/image_impl_test/coverage.dat
	@echo $(fNorm)
