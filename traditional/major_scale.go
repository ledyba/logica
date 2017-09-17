package traditional

import "math"

type MajorScale struct {
	base float64
	d    int
}

func NewMajorScale(base float64, d int) Scale {
	m := &MajorScale{
		base: base,
		d:    d,
	}
	return m
}

func CMajor() Scale {
	return NewMajorScale(440, -9)
}

func (m *MajorScale) MakeNote(deg int) *Note {
	note := &Note{}
	note.Freq = m.base * math.Pow(2, float64(m.d+deg)/12.0)
	return note
}
