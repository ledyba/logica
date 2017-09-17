.PHONY: all play video test claen

all:
	go build "github.com/ledyba/logica/..."

SCORE := ex2
SRCS := $(shell find . -type d -name scores -prune -o -type f -name \*.go)
SRCS += $(shell find scores/$(SCORE) -type f -name \*.go)

.bin:
	mkdir -p .bin

score.wav: $(SRCS)
	go build -o .bin/$(SCORE) "github.com/ledyba/logica/scores/$(SCORE)"
	.bin/$(SCORE) > score.wav

score.mp4: score.wav
	ffmpeg -y -loop 1 -i image.png -i score.wav -c:v libx264 -tune stillimage -c:a aac -b:a 192k -pix_fmt yuv420p -shortest score.mp4

play: score.wav
	play -t wav score.wav

video: score.mp4;

test:
	go test "github.com/ledyba/logica/..."

clean:
	rm -rfv .bin score.wav score.mp4
