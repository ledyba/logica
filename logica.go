package logica

import (
	"encoding/binary"
	"io"

	log "github.com/Sirupsen/logrus"
)

type StreamSpec struct {
	Channels   uint
	SampleRate uint
}

func (spec *StreamSpec) timeOf(idx uint) float64 {
	return float64(idx) / float64(spec.SampleRate)
}

type Stream interface {
	calc(spec *StreamSpec, from uint, buff []float32)
}

func writeHeader(spec *StreamSpec, out io.Writer, length float64) {
	headerLen := uint((4 + 4) + 4 + 4 + (4 + 16) + (4 + 4))
	bodyLen := uint(length * float64(spec.Channels*spec.SampleRate*2))
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
func Play(spec *StreamSpec, stream Stream, out io.Writer, length float64) {
	writeHeader(spec, out, length)
	endless := length < 0
	endIdx := uint(length * float64(spec.Channels*spec.SampleRate))

	fbuf := make([]float32, int(5*spec.Channels*spec.SampleRate))
	buf := make([]byte, len(fbuf)*2)

	idx := uint(0)
	for idx < endIdx || endless {
		fbufMax := uint(len(fbuf))
		if !endless && endIdx-idx < fbufMax {
			fbufMax = endIdx - idx
		}
		stream.calc(spec, idx, fbuf[:fbufMax])
		for i, f := range fbuf[:fbufMax] {
			binary.LittleEndian.PutUint16(buf[i*2:], uint16(int16(f*(65536/2-1))))
		}
		idx += fbufMax
		out.Write(buf[:fbufMax*2])
	}
	if !endless && idx != endIdx {
		log.Fatalf("Buffer index does not match: %d vs %d", idx, endIdx)
	}
}
