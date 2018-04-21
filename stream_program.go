package logica

import log "github.com/Sirupsen/logrus"

type PassiveProgramStream func(spec *StreamSpec, t float64, buff []float32)

func (f PassiveProgramStream) Calc(spec *StreamSpec, offset int, buff []float32) {
	end := len(buff)
	for idx := 0; idx < end; idx += int(spec.Channels) {
		f(spec, spec.TimeOf(offset+idx), buff[idx:idx+int(spec.Channels)])
	}
	if int(end) != len(buff) {
		log.Fatalf("Buffer size mismatch: %v != %v", len(buff), end)
	}
}

func (f PassiveProgramStream) Duration() float64 {
	return -1
}
