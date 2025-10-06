default:
	cargo run scripts/default
	display pic.png

custom:
	cargo run scripts/custom
	display pic.png

clean:
	cargo clean
	rm -rf *.ppm *.png
