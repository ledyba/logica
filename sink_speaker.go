package logica

import (
	"encoding/binary"
	"math"

	log "github.com/Sirupsen/logrus"
	"github.com/hajimehoshi/oto"
)

type speakerSink struct {
	spec   *StreamSpec
	player *oto.Player
}

func NewSpeakerSink(spec *StreamSpec, bufSizeMs float64) (Sink, error) {
	bufSize := int(bufSizeMs*float64(spec.SampleRate)*float64(spec.Channels)/1000.0) * 2
	player, err := oto.NewPlayer(int(spec.SampleRate), int(spec.Channels), 2, bufSize)
	if err != nil {
		return nil, err
	}
	return &speakerSink{
		spec:   spec,
		player: player,
	}, nil
}
func (sink *speakerSink) Close() {
	sink.player.Close()
}

func (sink *speakerSink) Play(stream Stream, offset, duration float64) {
	var err error
	if duration <= 0 {
		duration = stream.Duration()
	}
	spec := sink.spec
	endless := duration < 0
	endIdx := int(math.Ceil(duration*float64(spec.SampleRate))) * int(spec.Channels)

	fbuf := make([]float32, int(spec.SampleRate/100)*int(spec.Channels))
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
		_, err = sink.player.Write(buf[:fbufMax*2])
		if err != nil {
			log.Fatal(err)
		}
	}
	if !endless && idx != endIdx {
		log.Fatalf("Buffer index does not match: %d vs %d", idx, endIdx)
	}
}
