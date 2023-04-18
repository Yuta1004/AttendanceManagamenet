DIST_BIN := cgi-bin/attendance_management

$(DIST_BIN):
	mkdir -p cgi-bin
	cargo build
	cp target/debug/attendance_management $(DIST_BIN)

run: $(DIST_BIN)
	python3 -m http.server --cgi $(RUN_OPTS)
