package logica

import (
	"github.com/ledyba/logica/speaker"
)

type speakerSink struct {
	spec *StreamSpec
	jack speaker.Jack
}

func NewSpeakerSink(spec *StreamSpec, bufSizeMs float64) (Sink, error) {
	buffSize := int(bufSizeMs*float64(spec.SampleRate)*float64(spec.Channels)/1000.0) * 2
	jack, err := speaker.CreateJack(int(spec.Channels), int(spec.SampleRate), buffSize)
	if err != nil {
		return nil, err
	}
	return &speakerSink{
		spec: spec,
		jack: jack,
	}, nil
}
func (sink *speakerSink) Close() {
	sink.jack.Close()
}

func (sink *speakerSink) Play(stream Stream, offset, duration float64) {
	sink.jack.Start()
}
