.PHONY: all play test

all: score.wav ;

score.wav: score.js
	`npm bin`/babel-node score.js > score.wav

score.mp4: score.wav
	ffmpeg -loop 1 -i image.png -i score.wav -c:v libx264 -tune stillimage -c:a aac -b:a 192k -pix_fmt yuv420p -shortest score.mp4

play: score.wav
	play -t wav score.wav

test:
	`npm bin`/babel-node --presets es2015 test/test.js
