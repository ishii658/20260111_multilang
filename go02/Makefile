# Makefile for Go project

# Variables
BINARY_NAME=go02
BUILD_DIR=build
DEBUG_DIR=$(BUILD_DIR)/debug
RELEASE_DIR=$(BUILD_DIR)/release

# Go build tags
DEBUG_TAGS=debug
RELEASE_TAGS=release

# Build flags
GO_BUILD_FLAGS=-ldflags="-s -w -extldflags '-static'"

# Default target
.PHONY: all
all: debug

# Debug build target
.PHONY: debug
debug: $(DEBUG_DIR)/$(BINARY_NAME)

$(DEBUG_DIR)/$(BINARY_NAME):
	mkdir -p $(DEBUG_DIR)
	go build -o $(DEBUG_DIR)/$(BINARY_NAME) -tags=$(DEBUG_TAGS) $(GO_BUILD_FLAGS) .

# Release build target
.PHONY: release
release: $(RELEASE_DIR)/$(BINARY_NAME)

$(RELEASE_DIR)/$(BINARY_NAME):
	mkdir -p $(RELEASE_DIR)
	go build -o $(RELEASE_DIR)/$(BINARY_NAME) -tags=$(RELEASE_TAGS) $(GO_BUILD_FLAGS) .

# Clean build artifacts
.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)

# Install dependencies
.PHONY: deps
deps:
	go mod tidy

# Run the debug version
.PHONY: run-debug
run-debug: debug
	$(DEBUG_DIR)/$(BINARY_NAME)

# Run the release version
.PHONY: run-release
run-release: release
	$(RELEASE_DIR)/$(BINARY_NAME)

# Run tests
.PHONY: test
test:
	go test ./...

# Format code
.PHONY: fmt
fmt:
	go fmt ./...

# Vet code
.PHONY: vet
vet:
	go vet ./...

# All checks
.PHONY: check
check: fmt vet test

# Help target
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  debug        - Build debug version"
	@echo "  release      - Build release version"
	@echo "  clean        - Clean build artifacts"
	@echo "  deps         - Install dependencies"
	@echo "  run-debug    - Run debug version"
	@echo "  run-release  - Run release version"
	@echo "  test         - Run tests"
	@echo "  fmt          - Format code"
	@echo "  vet          - Vet code"
	@echo "  check        - Run all checks"
	@echo "  help         - Show this help"
