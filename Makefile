SRCS := $(shell find src/*.rs src/**/*.rs)
CGI_DIR := cgi-bin
CGI_BIN := $(CGI_DIR)/attendance_management

$(CGI_BIN): $(SRCS)
	mkdir -p cgi-bin
	cargo build
	cp target/debug/attendance_management $(CGI_BIN)
	if [ ! -e $(CGI_DIR)/data.json ]; then cp schema.json $(CGI_DIR)/data.json; fi

run: $(DIST_BIN)
	python3 -m http.server --cgi $(RUN_OPTS)

dist: $(DIST_BIN)
	cp -r cgi-bin dist
	cp schema.json dist/data.json
