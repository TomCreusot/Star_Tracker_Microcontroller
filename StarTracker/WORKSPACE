load("@bazel_tools//tools/build_defs/repo:git.bzl", "git_repository", "new_git_repository")
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")


http_archive(
    name = "rules_python",
    sha256 = "aa96a691d3a8177f3215b14b0edc9641787abaaa30363a080165d06ab65e1161",
    url = "https://github.com/bazelbuild/rules_python/releases/download/0.0.1/rules_python-0.0.1.tar.gz",
)

load("@rules_python//python:repositories.bzl", "py_repositories")

py_repositories()

# Only needed if using the packaging rules.
load("@rules_python//python:pip.bzl", "pip_repositories")

pip_repositories()

git_repository(
    name = "gtest",
    commit = "703bd9caab50b139428cea1aaff9974ebee5742e",
    remote = "https://github.com/abseil/googletest.git",
)


http_archive(
	name = "net_sourceforge_easybmp",
	urls = ["http://prdownloads.sourceforge.net/easybmp/EasyBMP_1.06.zip"],
	sha256 = "1ef19cc92b18a8ab272bf68a4ce8ce862d08208d4e675560c33fbd04d997c469", # Dont question it, bazel told me to put this in, it now works?
	# remote = "https://github.com/aburgh/EasyBMP",
	build_file_content = """
# package(default_visibility = ["//visibility:public"])
cc_library(
	name = "EasyBMP",
	srcs = ["EasyBMP.cpp"],
	hdrs = glob(["*.h"]),
	visibility = ["//visibility:public"],
)
"""
)
