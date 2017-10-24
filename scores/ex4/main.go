package main

import (
	"math"
	"os"

	log "github.com/Sirupsen/logrus"
	"github.com/ledyba/logica"
	"github.com/ledyba/logica/traditional"
)

func makeNoteStream(freq, duration float64) logica.Stream {
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
		v = float32(math.Sin(t * freq * 2.5 * pi2))
		v = float32(float64(v)*math.Exp(-t/duration)) * 0.7
		buff[0] = v
		buff[1] = v
	}
	return logica.ProgramStreamPassive(stream)
}

func main() {
	log.SetOutput(os.Stderr)
	scale := traditional.NewMajorScale(440, -2)
	score := traditional.NewScore(140, scale)

	t := score.NewTrack(0)
	log.Info(math.Sin(2*math.Pi), math.Sin(2*math.Pi*1000*10000))
	//t.Note(+0, 1)
	//t.Sharp(+0, 1)
	//t.Note(-3, 1)
	t.Raw(+6, 1)
	t.Raw(+9, 1)
	t.Raw(+7, 1)

	t.Note(+3, 1)
	t.Note(+3, 1)
	t.Note(+7, 1)
	t.Rest(0.5)

	t.Rest(1000)
	t.Close()

	//t = score.NewTrack(0)

	//t.Close()

	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}

	mix := logica.NewMixingStream()
	for i, track := range score.Tracks {
		log.Infof("Track %d", i)
		for _, note := range track.Notes {
			off := score.CalcOffset(note.Offset)
			duration := score.CalcDuration(note.Beats)
			mix.Mix(makeNoteStream(note.Freq.AsFloat(), duration), off, duration, 1)
		}
	}
	logica.Play(spec, mix, os.Stdout, 0.3, 0, -1)
}
