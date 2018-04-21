package main

import (
	"os"

	log "github.com/Sirupsen/logrus"
	"github.com/ledyba/logica"
)

func main() {
	log.SetOutput(os.Stderr)

	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}

	sink, err := logica.NewSpeakerSink(spec)
	if err != nil {
		log.Fatal(err)
	}
	defer sink.Close()
	//sink.Play(stream, 0, 0)
}
