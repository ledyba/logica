package main

import (
	"os"

	"math"

	"context"

	log "github.com/Sirupsen/logrus"
	"github.com/ledyba/joystick"
	"github.com/ledyba/logica"
)

var pi2 = math.Pi * 2

var update = make(chan struct{})
var ax, ay, bx, by float64

func updateLoop(ctx context.Context) {
	var err error
	js, err := joystick.Open(0)
	defer js.Close()
	if err != nil {
		log.Fatal(err)
	}
	defer js.Close()
	for {
		select {
		case <-ctx.Done():
			return
		case <-update:
			state, err := js.Read()
			if err != nil {
				log.Fatal(err)
			}
			ax = float64(state.AxisData[0]) / 32768.0
			ay = -float64(state.AxisData[1]) / 32768.0
			bx = float64(state.AxisData[2]) / 32768.0
			by = -float64(state.AxisData[3]) / 32768.0
		}
	}
}

func stream(_ *logica.StreamSpec, t float64, buff []float32) {
	vol := math.Sqrt(ax*ax+ay*ay) * 0.2
	baseFreq := math.Pow(2, 1+math.Atan2(ay, ax)/math.Pi) * 220.0
	modFreq := math.Pow(3.5, 1+math.Atan2(ay, ax)/math.Pi) * baseFreq
	v := vol * math.Sin(t*pi2*baseFreq+10*math.Sin(t*pi2*modFreq))
	in := float32(v)
	buff[0] = in
	buff[1] = in
	update <- struct{}{}
}

func main() {
	log.SetOutput(os.Stderr)
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()
	go updateLoop(ctx)

	spec := &logica.StreamSpec{
		Channels:   2,
		SampleRate: 44100,
	}

	sink, err := logica.NewSpeakerSink(spec, 1000)
	if err != nil {
		log.Fatal(err)
	}
	defer sink.Close()
	sink.Play(logica.PassiveProgramStream(stream), 0, 0)
}
