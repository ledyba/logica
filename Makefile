.PHONY: all play video test claen

all: score.wav ;

SRCS := $(shell find . -type f -name \*.go)

.bin:
	mkdir -p .bin

score.wav: $(SRCS)
	go build -o .bin/ex1 "github.com/ledyba/logica/scores/ex1"
	.bin/ex1 > score.wav

score.mp4: score.wav
	ffmpeg -loop 1 -i image.png -i score.wav -c:v libx264 -tune stillimage -c:a aac -b:a 192k -pix_fmt yuv420p -shortest score.mp4

play: score.wav
	play -t wav score.wav

video: score.mp4;

test:
	go test "github.com/ledyba/logica/..."

clean:
	rm -rfv .bin score.wav score.mp4
