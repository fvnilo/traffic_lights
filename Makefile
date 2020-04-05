build:
	docker build -t traffic-lights .
	docker run --rm -v ${CURDIR}/target:/app/target traffic-lights
