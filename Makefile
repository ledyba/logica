.PHONY: run test get claen

PKG=github.com/ledyba/logica

SCORE := ex3

SRCS := $(shell find . -type d -name scores -prune -o -type f -name \*.go) \
        $(shell find scores/$(SCORE) -type f -name \*.go)

run: .bin/$(SCORE)
	.bin/$(SCORE)

.bin/$(SCORE): $(SRCS)
	mkdir -p .bin
	go build -o .bin/$(SCORE) "$(PKG)/scores/$(SCORE)"

test:
	go test "$(PKG)/..."

get:
	go get -u "github.com/hajimehoshi/oto"
	go get -u "github.com/Sirupsen/logrus"
	#go get -u "github.com/ledyba/joystick"

clean:
	rm -rfv .bin
