.PHONY: run test get claen

PKG="github.com/ledyba/logica"

SCORE := ex2

SRCS := $(shell find . -type d -name scores -prune -o -type f -name \*.go) \
        $(shell find scores/$(SCORE) -type f -name \*.go)

run: .bin/$(SCORE)
	.bin/$(SCORE)

.bin/$(SCORE):
	mkdir -p .bin
	go build -o .bin/$(SCORE) "$(PKG)/scores/$(SCORE)"

test:
	go test "$(PKG)/..."

get:
	go get -u "github.com/hajimehoshi/oto"
	go get -u "github.com/Sirupsen/logrus"

clean:
	rm -rfv .bin
