package logica

import (
	"encoding/binary"
	"io"

	"math"

	log "github.com/Sirupsen/logrus"
)

type StreamSpec struct {
	Channels   uint
	SampleRate uint
}

func (spec *StreamSpec) TimeOf(idx int) float64 {
	return float64(idx) / float64(spec.SampleRate*spec.Channels)
}

func (spec *StreamSpec) ToIdx(offset float64) int {
	return int(offset*float64(spec.SampleRate)) * int(spec.Channels)
}

type Stream interface {
	Calc(spec *StreamSpec, from int, buff []float32)
	Duration() float64
}

func writeHeader(spec *StreamSpec, out io.Writer, length float64) {
	headerLen := uint((4 + 4) + 4 + 4 + (4 + 16) + (4 + 4))
	bodyLen := uint(length*float64(spec.SampleRate)) * spec.Channels * 2
	if length < 0 {
		bodyLen = 0
	}
	out.Write([]byte{'R', 'I', 'F', 'F'})
	binary.Write(out, binary.LittleEndian, int32(headerLen+bodyLen-8))
	out.Write([]byte{'W', 'A', 'V', 'E'})
	out.Write([]byte{'f', 'm', 't', ' '})
	binary.Write(out, binary.LittleEndian, int32(16))
	binary.Write(out, binary.LittleEndian, int16(1))
	binary.Write(out, binary.LittleEndian, int16(spec.Channels))
	binary.Write(out, binary.LittleEndian, int32(spec.SampleRate))
	binary.Write(out, binary.LittleEndian, int32(spec.Channels*spec.SampleRate*2))
	binary.Write(out, binary.LittleEndian, int16(spec.Channels*2))
	binary.Write(out, binary.LittleEndian, int16(16))
	out.Write([]byte{'d', 'a', 't', 'a'})
	binary.Write(out, binary.LittleEndian, int32(bodyLen))

}

// Play ...
func Play(spec *StreamSpec, stream Stream, out io.Writer, scale float32, offset, duration float64) {
	if duration < 0 {
		duration = stream.Duration()
	}
	writeHeader(spec, out, duration)
	endless := duration < 0
	endIdx := int(math.Ceil(duration*float64(spec.SampleRate))) * int(spec.Channels)

	fbuf := make([]float32, int(5*spec.SampleRate)*int(spec.Channels))
	buf := make([]byte, len(fbuf)*2)

	idx := spec.ToIdx(offset)
	for idx < endIdx || endless {
		fbufMax := len(fbuf)
		if !endless && endIdx-idx < fbufMax {
			fbufMax = endIdx - idx
		}
		stream.Calc(spec, idx, fbuf[:fbufMax])
		for i, f := range fbuf[:fbufMax] {
			f *= scale
			if f > 1.0 {
				log.Errorf("Overlevel: %v at %f.02[sec]", f, spec.TimeOf(i+idx))
				f = 1.0
			} else if f < -1.0 {
				log.Errorf("Overlevel: %v at %f.02[sec]", f, spec.TimeOf(i+idx))
				f = -1.0
			}
			binary.LittleEndian.PutUint16(buf[i*2:], uint16(int16(f*(65536/2-1))))
		}
		idx += fbufMax
		out.Write(buf[:fbufMax*2])
	}
	if !endless && idx != endIdx {
		log.Fatalf("Buffer index does not match: %d vs %d", idx, endIdx)
	}
}
