package main

import (
	"os"

	"math"

	"github.com/ledyba/logica"
)

// セミ

func f(base, b, m, t float64) float64 {
	pi2 := 2 * math.Pi
	freq := base + b*math.Sin(m*pi2*t)

	return math.Sin(freq * t * pi2)
}

func stream(_ *logica.StreamSpec, t float64, buff []float32) {
	cfreq := 220.0
	mfreq := cfreq * 2

	v := f(cfreq, 1, mfreq, t+100000000)

	buff[0] = float32(v / 2)
	buff[1] = float32(v / 2)
}

func main() {
	stream := logica.ProgramStreamPassive(stream)
	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}
	logica.Play(spec, stream, os.Stdout, 0.8, 0, 7)
}
