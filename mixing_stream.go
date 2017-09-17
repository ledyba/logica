package logica

import (
	"sort"

	"math"

	log "github.com/Sirupsen/logrus"
)

type mixing struct {
	stream   Stream
	offset   float64
	duration float64
	scale    float32
}
type MixingStream struct {
	mixes []mixing
}

func NewMixingStream() *MixingStream {
	stream := &MixingStream{
		mixes: make([]mixing, 0),
	}
	return stream
}

func (m *MixingStream) mixingSorter(i, j int) bool {
	return m.mixes[i].offset < m.mixes[j].offset
}

func (m *MixingStream) Sort() {
	sort.Slice(m.mixes, m.mixingSorter)
}

func (m *MixingStream) Calc(spec *StreamSpec, from int, chunk []float32) {
	m.Sort()
	chunkBegIdxAbs := from
	chunkEndIdxAbs := from + len(chunk)
	for i := range chunk {
		chunk[i] = 0
	}
	buff := make([]float32, len(chunk))

	for _, mix := range m.mixes {
		mixBegIdxAbs := spec.ToIdx(mix.offset)
		mixEndIdxAbs := spec.ToIdx(mix.offset + mix.duration)
		mixDurationIdx := mixEndIdxAbs - mixBegIdxAbs
		if mixBegIdxAbs >= chunkEndIdxAbs {
			break /* because this mix list is ensured to be sorted by beg idx. */
		}
		if chunkBegIdxAbs >= mixEndIdxAbs {
			continue
		}

		begInBuff := mixBegIdxAbs - chunkBegIdxAbs
		endInBuff := mixEndIdxAbs - chunkBegIdxAbs
		begInMix := chunkBegIdxAbs - mixBegIdxAbs

		if begInBuff < 0 {
			begInBuff = 0
		}
		if endInBuff > len(buff) {
			endInBuff = len(buff)
		}

		if begInMix < 0 {
			begInMix = 0
		}

		if begInMix > mixDurationIdx {
			log.Fatalf("[BUG] Mixing area overlap: %v > %v", begInMix, mixDurationIdx)
		}

		b := buff[begInBuff:endInBuff]
		for i := range b {
			b[i] = 0
		}
		mix.stream.Calc(spec, begInMix, b)
		for i, v := range b {
			chunk[i+begInBuff] += v * mix.scale
		}
	}
}

func (m *MixingStream) Duration() float64 {
	d := -1.0
	for _, mix := range m.mixes {
		if mix.duration < 0 {
			continue
		}
		d = math.Max(mix.duration+mix.offset, d)
	}
	return d
}

func (m *MixingStream) Mix(other Stream, offset, duration float64, scale float32) *MixingStream {
	param := mixing{
		stream:   other,
		offset:   offset,
		duration: duration,
		scale:    scale,
	}
	m.mixes = append(m.mixes, param)
	return m
}
