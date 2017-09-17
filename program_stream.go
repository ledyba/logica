package logica

type ProgramStreamPassive func(spec *StreamSpec, t float64, buff []float32)

func (f ProgramStreamPassive) calc(spec *StreamSpec, from uint, buff []float32) {
	end := uint(len(buff))
	for idx := uint(0); idx < end; idx += spec.Channels {
		f(spec, spec.timeOf(from+idx), buff[idx:(idx+spec.Channels)])
	}
}
