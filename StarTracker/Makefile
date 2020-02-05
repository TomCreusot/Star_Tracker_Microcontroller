fNorm="\e[0m"
fBold="\e[1mBold"
fRed="\033[0;31m"
fBRed="\e[41m"
fBDGrey="\e[100m"

testToRun=//libs/star_tracker:star_tracker

all:
	bazel test $(testToRun) --test_output=all
	bazel coverage $(testToRun)
	@echo $(fBDGrey)
	@lcov --summary bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)


test:
ifndef LCOV
	echo Util
	bazel test //libs/Utils:PointTest --test_output=errors

	echo ImageProcessing
	bazel test //libs/ImageProcessing:BlobTest --test_output=errors
	bazel test //libs/ImageProcessing/nix:ImageTest --test_output=errors
	bazel test //libs/ImageProcessing/stm:ImageTest --test_output=errors
	bazel test //libs/ImageProcessing:ImageBaseTest --test_output=errors

	echo StarTracker
	bazel test //libs/StarTracker:StarTrackerTest --test_output=errors
	bazel test //libs/StarTracker:ComboTest --test_output=errors


else
	@ echo Util
	bazel coverage //libs/Utils:PointTest
	@echo $(fBDGrey)
	lcov --summary bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)

	bazel coverage //libs/Utils:ImageTest
	@echo $(fBDGrey)
	 @lcov --summary bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)



	@ echo "\n\n\n"ImageProcessing
	bazel coverage //libs/ImageProcessing:ImageBaseTest
	@echo $(fBDGrey)
	@lcov --summary bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)
	bazel coverage //libs/ImageProcessing/nix:ImageTest
	@echo $(fBDGrey)
	@lcov --summary bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)
	bazel coverage //libs/ImageProcessing/stm:ImageTest
	@echo $(fBDGrey)
	@lcov --summary bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)

	bazel coverage //libs/ImageProcessing:BlobTest
	@echo $(fBDGrey)
	@lcov --summary bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)

	 @ echo "\n\n\n" StarTracker
	bazel coverage //libs/StarTracker:StarTrackerTest
	@echo $(fBDGrey)
	@lcov --summary bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)

	bazel coverage //libs/StarTracker:ComboTest
	@echo $(fBDGrey)
	@lcov --summary bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)
endif
