package traditional

import (
	"math"

	log "github.com/Sirupsen/logrus"
)

type MajorScale struct {
	base float64
	d    int
	name [7]int
}

// @see: http://www.moge.org/okabe/temp/scale.pdf
func NewMajorScale(base float64, d int) Scale {
	m := &MajorScale{
		base: base,
		d:    d,
		name: [7]int{0, 2, 4, 5, 7, 9, 11},
	}
	return m
}

// @see: http://www.moge.org/okabe/temp/scale.pdf
func NewMinorScale(base float64, d int) Scale {
	m := &MajorScale{
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

func (m *MajorScale) MakeNote(tone int) *Note {
	note := &Note{}
	div := tone / 7
	mod := tone % 7
	if mod < 0 {
		mod += 7
		div -= 1
		if mod+div*7 != tone {
			log.Fatalf("Not match: mod(%d) + div(%d) * 7 != %v", mod, div, tone)
		}
	}
	deg := div*12 + m.name[mod]
	// @see: http://drumimicopy.com/audio-frequency/
	note.Freq = m.base * math.Pow(2, (float64(m.d+deg))/12.0)
	return note
}
