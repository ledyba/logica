.PHONY: all test

all:
	`npm bin`/babel-node score.js > score.wav

test:
	`npm bin`/babel-node --presets es2015 test/test.js
