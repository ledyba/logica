package logica

import (
	"math"
	"encoding/binary"
	log "github.com/Sirupsen/logrus"
	"io"
)

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

type waveSink struct {
	spec *StreamSpec
	out io.Writer
}

func NewWaveSink(spec *StreamSpec, out io.Writer) Sink {
	return &waveSink{
		spec: spec,
		out: out,
	}
}
func (sink *waveSink) Close() {
}

func (sink *waveSink) Play(stream Stream, offset, duration float64) {
	if duration < 0 {
		duration = stream.Duration()
	}
	spec := sink.spec
	writeHeader(spec, sink.out, duration)
	endless := duration < 0
	endIdx := int(math.Ceil(duration*float64(spec.SampleRate))) * int(spec.Channels)

	fbuf := make([]float32, int(5*spec.SampleRate)*int(spec.Channels))
	buf := make([]byte, len(fbuf)*2)

	idx := spec.IndexOf(offset)
	for idx < endIdx || endless {
		fbufMax := len(fbuf)
		if !endless && endIdx-idx < fbufMax {
			fbufMax = endIdx - idx
		}
		stream.Calc(spec, idx, fbuf[:fbufMax])
		for i, f := range fbuf[:fbufMax] {
			if f > 1.0 {
				log.Errorf("Sound cracking: %.4f at %.3f[sec]", f, spec.TimeOf(i+idx))
				f = 1.0
			} else if f < -1.0 {
				log.Errorf("Sound cracking: %.4f at %.3f[sec]", f, spec.TimeOf(i+idx))
				f = -1.0
			}
			binary.LittleEndian.PutUint16(buf[i*2:], uint16(int16(f*(65536/2-1))))
		}
		idx += fbufMax
		sink.out.Write(buf[:fbufMax*2])
	}
	if !endless && idx != endIdx {
		log.Fatalf("Buffer index does not match: %d vs %d", idx, endIdx)
	}
}
