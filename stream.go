package logica

type StreamSpec struct {
	Channels   uint
	SampleRate uint
}

func (spec *StreamSpec) TimeOf(idx int) float64 {
	return float64(idx) / float64(spec.SampleRate*spec.Channels)
}

func (spec *StreamSpec) IndexOf(offset float64) int {
	return int(offset*float64(spec.SampleRate)) * int(spec.Channels)
}

type Stream interface {
	Calc(spec *StreamSpec, from int, buff []float32)
	Duration() float64
}
