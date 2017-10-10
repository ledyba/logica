package main

import (
	"math"
	"os"

	"github.com/ledyba/logica"
	"github.com/ledyba/logica/traditional"
)

func makeNoteStream(freq float64) logica.Stream {
	stream := func(_ *logica.StreamSpec, t float64, buff []float32) {
		pi2 := math.Pi * 2

		//f := freq + 0.5*math.Sin(t*freq*3.5*pi2)*math.Exp(-(t))
		//v := float32(math.Sin(t*pi2*f) * math.Exp(-t))
		v := float32(0)
		_, f := math.Modf(t * freq)
		duty := 0.5 + 0.2*math.Sin(t*freq*3.5*pi2)*math.Exp(-(t+1))
		if f < duty {
			v = -1
		} else {
			v = 1
		}
		v = float32(float64(v) * math.Exp(-t))
		buff[0] = v
		buff[1] = v
	}
	return logica.ProgramStreamPassive(stream)
}

func main() {
	scale := traditional.NewMajorScale(440, 0)
	score := traditional.NewScore(180, scale)
	t := score.NewTrack(0)

	t.Note(0, 1).Ok()
	t.Note(-3, 1).Ok()
	t.Note(0, 2).Ok()

	t.Note(1, 2).Ok()
	t.Note(3, 2).Note(-3, 1).Note(-2, 1).Done(2)

	t.Note(2, .75).Note(-2, .75).Ok()
	t.Note(3, .75).Ok()
	t.Note(2, .75).Ok()

	t.Note(1, 1).Note(-3, 1).Ok()
	t.Note(0, 1).Ok()

	t.Note(1, 2).Ok()
	t.Note(-1, 2).Note(-4, 2).Ok()

	t.Note(0, 1).Ok()
	t.Note(1, 1).Ok()
	t.Note(2, 2).Ok()

	t.Note(3, 2).Ok()
	t.Note(2, 1).Ok()
	t.Note(1, 1).Ok()

	t.Note(4, 2).Ok()
	t.Note(3, 1).Ok()
	t.Note(2, 1).Ok()
	t.Note(1, 2).Ok()
	t.Close()

	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}
	mix := logica.NewMixingStream()
	score.Sort()
	for _, note := range score.Notes {
		mix.Mix(makeNoteStream(note.Freq), note.Offset, note.Duration, 1)
	}
	mix.Sort()

	logica.Play(spec, mix, os.Stdout, 0.25, 0, -1)
}
