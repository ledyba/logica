package logica

import log "github.com/Sirupsen/logrus"

type ProgramStreamPassive func(spec *StreamSpec, t float64, buff []float32)

func (f ProgramStreamPassive) Calc(spec *StreamSpec, from int, buff []float32) {
	end := len(buff)
	for idx := 0; idx < end; idx += int(spec.Channels) {
		f(spec, spec.TimeOf(from+idx), buff[idx:idx+int(spec.Channels)])
	}
	if int(end) != len(buff) {
		log.Fatalf("Buffer size mismatch: %v != %v", len(buff), end)
	}
}

func (f ProgramStreamPassive) Duration() float64 {
	return -1
}
