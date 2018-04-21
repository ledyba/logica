package main

import (
	"os"

	"math"

	"math/rand"

	log "github.com/Sirupsen/logrus"
	"github.com/ledyba/logica"
)

// random walk

var vec [100]float64
var pi2 = math.Pi * 2

func f(t float64, offset int, level int) (float64, int) {
	if level == 0 {
		return vec[offset], 1
	}
	origOffset := offset

	base := vec[offset]
	offset++

	w1 := vec[offset]
	offset++
	m1, used := f(t, offset, level-1)
	offset += used

	v := base + w1*math.Sin(m1*pi2*t)
	return v, offset - origOffset
}

func stream(_ *logica.StreamSpec, t float64, buff []float32) {

	//walk
	for i := range vec {
		vec[i] += rand.NormFloat64() / 10000
	}

	freq, used := f(t, 0, 3)
	v := math.Sin(freq * pi2 * t)
	log.Info(vec[0:used])

	buff[0] = float32(v / 2)
	buff[1] = float32(v / 2)
}

func main() {
	stream := logica.PassiveProgramStream(stream)
	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}
	sink := logica.NewWaveSink(spec, os.Stdout)
	defer sink.Close()
	vec[0] = 440
	sink.Play(stream, 0, 100)
}
