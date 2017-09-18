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
	score := traditional.NewScore(180, traditional.NewMajorScale(440, 0))
	t := score.NewTrack(0)

	t.AddNote(1, 0)
	t.AddNote(1, -3)
	t.AddNote(2, 0)

	t.AddNote(2, 1)
	t.Fork(2, func(ts []*traditional.ScoreTrack) {
		ts[0].AddNote(2, 3)
		ts[1].AddNote(1, -3)
		ts[1].AddNote(1, -2)
	})

	t.AddNotes(.75, 2, -2)
	t.AddNote(.75, 3)
	t.AddNote(.75, 2)

	t.AddNotes(1, 1, -3)
	t.AddNote(1, 0)

	t.AddNote(2, 1)
	t.AddNotes(2, -1, -4)

	t.AddNote(1, 0)
	t.AddNote(1, 1)
	t.AddNote(2, 2)

	t.AddNote(2, 3)
	t.AddNote(1, 2)
	t.AddNote(1, 1)

	t.AddNote(2, 4)
	t.AddNote(1, 3)
	t.AddNote(1, 2)
	t.AddNote(2, 1)
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

	logica.Play(spec, mix, os.Stdout, 0.4, 0, -1)
}
