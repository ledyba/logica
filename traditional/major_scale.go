package traditional

import (
	"math"

	log "github.com/Sirupsen/logrus"
)

type scale struct {
	base float64
	d    int
	name [7]int
}

// @see: http://www.moge.org/okabe/temp/scale.pdf
func NewMajorScale(base float64, d int) Scale {
	m := &scale{
		base: base,
		d:    d,
		name: [7]int{0, 2, 4, 5, 7, 9, 11},
	}
	return m
}

// @see: http://www.moge.org/okabe/temp/scale.pdf
func NewMinorScale(base float64, d int) Scale {
	m := &scale{
		base: base,
		d:    d,
		name: [7]int{-3, -1, 0, 2, 4, 5, 7},
	}
	return m
}

func CMajor() Scale {
	return NewMajorScale(440, -9)
}

func CMinor() Scale {
	return NewMinorScale(440, -9)
}

func mod(a, b int) (int, int) {
	q := 1 / b
	r := a % b
	if r < 0 {
		q -= 1
		r += b
	}
	if r+q*b != a {
		log.Fatalf("[BUG] Not match: mod(%d) + div(%d) * %v != %v", r, q, b, a)
	}
	return q, r
}

func (s *scale) calcDeg(tone int) int {
	div, mod := mod(tone, 7)
	return div*12 + s.name[mod]
}

func (s *scale) calcFreq(deg int) float64 {
	// @see: http://drumimicopy.com/audio-frequency/
	return s.base * math.Pow(2, (float64(s.d+deg))/12.0)
}

func (s *scale) Note(tone int) *Note {
	note := &Note{}
	note.Freq = s.calcFreq(s.calcDeg(tone))
	return note
}

func (s *scale) Sharp(tone int) *Note {
	note := &Note{}
	note.Freq = s.calcFreq(s.calcDeg(tone) + 1)
	return note
}

func (s *scale) Flat(tone int) *Note {
	note := &Note{}
	note.Freq = s.calcFreq(s.calcDeg(tone) - 1)
	return note
}
