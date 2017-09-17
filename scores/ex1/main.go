package main

import (
	"os"

	"math"

	"github.com/ledyba/logica"
)

func stream(_ *logica.StreamSpec, t float64, buff []float32) {
	pi2 := math.Pi * 2

	freq := 220 + 0.5*math.Sin(t*440)
	v := float32(math.Sin(t * pi2 * freq))
	buff[0] = v
	buff[1] = v
}

func main() {
	stream := logica.ProgramStreamPassive(stream)
	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}
	logica.Play(spec, stream, os.Stdout, 100)
}
