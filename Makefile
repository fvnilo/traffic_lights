build:
	docker build -t traffic-lights .
	docker run --rm -v ${CURDIR}/target:/app/target traffic-lights

publish: build
	scp target/armv7-unknown-linux-gnueabihf/release/traffic_lights pi@${HOST}:/home/pi
