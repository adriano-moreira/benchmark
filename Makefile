all: build run
build:
	bash dockerBuild.sh
run:
	bash dockerRun.sh
