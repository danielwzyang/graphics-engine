RUN ?= WINIT_UNIX_BACKEND=x11 cargo run

default:
	${RUN} scripts/stuycs

custom:
	${RUN} scripts/github

clean:
	rm -rf *.ppm *.png
