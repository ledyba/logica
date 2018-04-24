package main

import (
	"log"
	"math"
	"os"

	"time"

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
		duty := 0.5 + 0.2*math.Sin(t*freq*2.5*pi2)*math.Exp(-(t+1))
		if f < duty {
			v = -1
		} else {
			v = 1
		}
		v = float32(float64(v)*math.Exp(-t)) * 0.2
		buff[0] = v
		buff[1] = v
	}
	return logica.PassiveProgramStream(stream)
}

func main() {
	log.SetOutput(os.Stderr)
	scale := traditional.NewMajorScale(440, 0)
	score := traditional.NewScore(180, scale)
	t := score.NewTrack(0)

	t.Note(0, 1)
	t.Note(-3, 1)
	t.Note(0, 2)

	t.Note(1, 2)
	t.Note(3, 2)

	t.Note(2, .75)
	t.Note(3, .75)
	t.Note(2, .75)

	t.Note(1, 1)
	t.Note(0, 1)

	t.Note(1, 2)
	t.Note(-1, 2)

	t.Note(0, 1)
	t.Note(1, 1)
	t.Note(2, 2)

	t.Note(3, 2)
	t.Note(2, 1)
	t.Note(1, 1)

	t.Note(4, 2)
	t.Note(3, 1)
	t.Note(2, 1)
	t.Note(1, 2)
	t.Close()

	t = score.NewTrack(0)
	t.Seek(6)
	t.Note(1, 1)
	t.Note(2, 1)
	t.Note(1, 2)

	t.Close()

	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}
	mix := logica.NewMixingStream()

	for _, track := range score.Tracks {
		for _, note := range track.Notes {
			off := score.CalcOffset(note.Offset)
			duration := score.CalcDuration(note.Beats)
			mix.Mix(makeNoteStream(note.Freq.AsFloat()), off, duration, 1)
		}
		mix.Sort()
	}

	sink, err := logica.NewSpeakerSink(spec, 1000)
	defer sink.Close()
	if err != nil {
		log.Fatal(err)
	}
	sink.Play(mix, 0, 0)
	// TODO:バッファが再生し終わる前にshutdownされてしまう
	time.Sleep(1000 * time.Millisecond)
}
