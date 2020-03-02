fBold="\e[1mBold"
fRed="\033[0;31m"
fBRed="\e[41m"
fBDGrey="\e[100m"


all:
	@ - rm -f -r demo
	bazel build //libs:demo
	mv bazel-bin/libs/demo .
	./demo Aldebaran_4h36m_16deg32m_10fov.bmp 0.9 6 20 7 10
	#./demo Archernar_1h37m_57deg14m_45fov.bmp 0.9 6 20 7 10

test:
	@echo $(fBold) $(fBRed)
	@ echo util
	@echo $(fNorm)

	bazel coverage //libs/util/... --combined_report=lcov
	bazel test //libs/util/... --test_output=all
	@echo $(fBDGrey)
	lcov --l bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)



	@echo $(fBold) $(fBRed)
	@ echo star_tracker
	@echo $(fNorm)

	bazel coverage //libs/star_tracker/... --combined_report=lcov
	bazel test //libs/star_tracker/... --test_output=all
	@echo $(fBDGrey)
	lcov --l bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)


	@echo $(fBold) $(fBRed)
	@ echo image_processing
	@echo $(fNorm)

	bazel coverage //libs/image_processing/... --combined_report=lcov
	bazel test //libs/image_processing/... --test_output=all
	@echo $(fBDGrey)
	lcov --l bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)


	@echo $(fBold) $(fBRed)
	@ echo get_image
	@echo $(fNorm)

	bazel coverage //libs/get_image/... --combined_report=lcov
	bazel test //libs/get_image/... --test_output=all
	@echo $(fBDGrey)
	lcov --l bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)



	@echo $(fBold) $(fBRed)
	@ echo database
	@echo $(fNorm)

	bazel coverage //libs/database/... --combined_report=lcov
	bazel test //libs/database/... --test_output=all
	@echo $(fBDGrey)
	lcov --l bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)


	@echo $(fBold) $(fBRed)
	@ echo coms
	@echo $(fNorm)

	bazel coverage //libs/coms/... --combined_report=lcov
	bazel test //libs/coms/... --test_output=all
	@echo $(fBDGrey)
	lcov --l bazel-out/_coverage/_coverage_report.dat
	@echo $(fNorm)
