RUN ?= WINIT_UNIX_BACKEND=x11 cargo run

default:
	${RUN} scripts/default
	display pic.png

custom:
	${RUN} scripts/custom
	display pic.png

clean:
	cargo clean
	rm -rf *.ppm *.png
