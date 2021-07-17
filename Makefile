default: build run convert

build:
	cargo build

run:
	cargo run

convert:
	# depends on image magick being installed
	./convert_images.sh

