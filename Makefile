.PHONY: all

all:
	@node test/test.js test.wav
	rm test.wav
