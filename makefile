RUN ?= WINIT_UNIX_BACKEND=x11 cargo run

default:
	${RUN} scripts/default

custom:
	${RUN} scripts/custom

clean:
	cargo clean
	rm -rf *.ppm *.png
