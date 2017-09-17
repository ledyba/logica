package main

import (
	"os"

	"math"

	log "github.com/Sirupsen/logrus"
	"github.com/ledyba/logica"
)

func solveFeedback(init float64, f func(float64) float64) float32 {
	prev := init
	for i := 0; i < 20; i++ {
		v := f(prev)
		if math.Abs(prev-v) < 0.01 {
			return float32(v)
		}
		prev = v
	}
	log.Fatalf("Diverged")
	return float32(math.NaN())
}

func stream(_ *logica.StreamSpec, t float64, buff []float32) {
	pi2 := math.Pi * 2

	freq := 220 + 0.5*math.Sin(t*220*3.5)
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
	logica.Play(spec, stream, os.Stdout, 0.8, 0, 100)
}
