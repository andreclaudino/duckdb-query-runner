BINARY_NAME := $$(cat Cargo.toml | grep name | head -n 1 | awk '{print $$3}' | sed -r 's/^"|"$$//g')
PROJECT_VERSION := $$(cat Cargo.toml | grep version | head -n 1 | awk '{print $$3}' | sed -r 's/^"|"$$//g')
GIT_REFERENCE := $$(git log -1 --pretty=%h)

IMAGE_REPOSITORY=docker.io/andreclaudino/$(BINARY_NAME)
IMAGE_NAME=$(IMAGE_REPOSITORY):$(PROJECT_VERSION)

flags/create:
	mkdir -p flags/flags
	touch flags/create


flags/build: flags/create
	podman build -t $(IMAGE_REPOSITORY) -f docker/Dockerfile . \
		--build-arg GIT_REFERENCE=$(GIT_REFERENCE) \
		--build-arg VERSION=$(PROJECT_VERSION) \
		--build-arg binary_name=${BINARY_NAME}
		--platform linux/amd64,linux/arm64

	touch flags/build


flags/login: flags/create
	podman login docker.io
	touch flags/login


flags/tag: flags/build
	podman tag $(IMAGE_REPOSITORY):latest $(IMAGE_NAME)
	touch flags/tag


flags/push: flags/login flags/tag
	podman push $(IMAGE_REPOSITORY):latest
	podman push $(IMAGE_NAME)


clean:
	rm -rf flags/
	podman rmi docker.io/andreclaudino/duckdb-query-runner:latest
	rm -rf $(CERTIFICATES_PATH)/create