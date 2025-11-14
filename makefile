RUN ?= WINIT_UNIX_BACKEND=x11 cargo run
DEFAULT ?= scripts/test
CUSTOM ?= scripts/dino

default:
	${RUN} ${DEFAULT}

custom:
	${RUN} ${CUSTOM}

# run with make run SCRIPT="path"
run:
	${RUN} ${SCRIPT}

clean:
	rm -rf *.ppm *.png
