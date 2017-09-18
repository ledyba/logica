package main

import (
	"os"

	"math"

	"github.com/ledyba/logica"
)

// セミ

func f(freq, t float64) float64 {
	v := 0.0
	pi2 := 2 * math.Pi
	_, f := math.Modf(t * freq)
	duty := 0.5 + 0.2*math.Sin(t*freq*3.5*pi2)*math.Exp(-(t+1))
	if f < duty {
		v = -1
	} else {
		v = 1
	}
	v = float64(v)
	return v
}

func stream(_ *logica.StreamSpec, t float64, buff []float32) {
	base := float64(440)
	pi2 := 2 * math.Pi
	freq := base + 5*math.Sin(pi2*t*math.Exp(-t))
	v := f(base, t) + f(freq, t+1.2)
	v /= 2
	buff[0] = float32(v)
	buff[1] = float32(v)
}

func main() {
	stream := logica.ProgramStreamPassive(stream)
	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}
	logica.Play(spec, stream, os.Stdout, 0.8, 0, 10)
}
