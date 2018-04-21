package main

import (
	"math"

	"github.com/ledyba/logica"
	"os"
)

var pi2 = math.Pi * 2

func stream(_ *logica.StreamSpec, t float64, buff []float32) {
	v := float32(math.Sin(t * pi2 * 220.0 + 10*math.Sin(t*pi2*220.0*3.5)) * 0.2)
	buff[0] = v
	buff[1] = v
}

func main() {
	stream := logica.PassiveProgramStream(stream)
	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}
	sink := logica.NewWaveSink(spec, os.Stdout)
	defer sink.Close()
	sink.Play(stream, 0, 100)
}
