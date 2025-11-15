RUN ?= WINIT_UNIX_BACKEND=x11 cargo run
DEFAULT ?= scripts/3dface.mdl
CUSTOM ?= scripts/dino.mdl

default:
	${RUN} ${DEFAULT}

custom:
	${RUN} ${CUSTOM}

# run with make run SCRIPT="path"
run:
	${RUN} ${SCRIPT}

clean:
	rm -rf *.ppm *.png
