RUN ?= WINIT_UNIX_BACKEND=x11 cargo run
CUSTOM ?= scripts/dino

default:
	${RUN} scripts/test

run:
	${RUN} ${SCRIPT}

custom:
	${RUN} ${CUSTOM}

clean:
	rm -rf *.ppm *.png
