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

		f := freq + 0.5*math.Sin(t*freq*3.5*pi2)*math.Exp(-(t))
		v := float32(math.Sin(t*pi2*f) * math.Exp(-t))
		buff[0] = v
		buff[1] = v
	}
	return logica.ProgramStreamPassive(stream)
}

func main() {
	score := traditional.NewScore(180, traditional.CMajor())
	t := score.NewTrack(0)
	t.AddNote(0, 1)
	t.AddNote(-3, 1)
	t.AddNote(0, 2)

	t.AddNote(1, 2)
	t.AddNote(3, 2)

	t.AddNote(2, .75)
	t.AddNote(3, .75)
	t.AddNote(2, .75)
	t.AddNote(1, 1)
	t.AddNote(0, 1)

	t.AddNote(1, 2)
	t.AddNote(-3, 2)

	t.AddNote(0, 1)
	t.AddNote(1, 1)
	t.AddNote(2, 2)

	t.AddNote(3, 2)
	t.AddNote(2, 1)
	t.AddNote(1, 1)

	t.AddNote(4, 2)
	t.AddNote(3, 1)
	t.AddNote(2, 1)
	t.AddNote(1, 2)
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

	logica.Play(spec, mix, os.Stdout, 0.8, 0, -1)
}
