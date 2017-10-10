package traditional

import (
	"math"
	"sort"
)

type Note struct {
	Offset     float64 /* in sec */
	Freq       float64
	Duration   float64 /* in sec */
	SeekOffset float64 /* in sec */
}

type Scale interface {
	Note(deg int) *Note
	Flat(deg int) *Note
	Sharp(deg int) *Note
}

type Score struct {
	Bpm   float64
	Scale Scale
	Notes []*Note
}

func NewScore(bpm float64, scale Scale) *Score {
	return &Score{
		Bpm:   bpm,
		Scale: scale,
		Notes: make([]*Note, 0),
	}
}

func (score *Score) scoreSorter(i, j int) bool {
	return score.Notes[i].Offset < score.Notes[j].Offset
}

func (score *Score) Sort() {
	sort.Slice(score.Notes, score.scoreSorter)
}

func (score *Score) NewTrack(offset float64) *ScoreTrack {
	return &ScoreTrack{
		score:  score,
		offset: offset,
		notes:  make([]*Note, 0),
	}
}

// ----------------------------------------------------------------------------

type ScoreTrack struct {
	score  *Score
	offset float64 /* in sec */
	notes  []*Note
}

func (track *ScoreTrack) With(note *Note, beats float64) *ChordBuilder {
	cb := &ChordBuilder{
		track:  track,
		offset: track.offset,
		notes:  make([]*Note, 0),
	}
	return cb.With(note, beats)
}

func (track *ScoreTrack) Note(tone int, beats float64) *ChordBuilder {
	return track.With(track.score.Scale.Note(tone), beats)
}

func (track *ScoreTrack) Flat(tone int, beats float64) *ChordBuilder {
	return track.With(track.score.Scale.Flat(tone), beats)
}

func (track *ScoreTrack) Sharp(tone int, beats float64) *ChordBuilder {
	return track.With(track.score.Scale.Sharp(tone), beats)
}

func (track *ScoreTrack) Rest(beats float64) {
	bpm := track.score.Bpm
	track.offset += beats / (bpm / 60.0)
}

func (track *ScoreTrack) Close() {
	score := track.score
	for _, pos := range track.notes {
		score.Notes = append(score.Notes, pos)
	}
	// Closed.
	track.score = nil
	track.offset = math.NaN()
	track.score = nil
}

// ----------------------------------------------------------------------------

type ChordBuilder struct {
	track    *ScoreTrack
	offset   float64 /* in sec */
	duration float64 /* in sec, longest */
	notes    []*Note
}

func (cb *ChordBuilder) With(note *Note, beats float64) *ChordBuilder {
	bpm := cb.track.score.Bpm
	note.Offset = cb.offset
	note.Duration = beats / (bpm / 60.0)
	note.SeekOffset = 0
	cb.notes = append(cb.notes, note)

	cb.duration = math.Max(cb.duration, note.Duration)

	return cb
}

func (cb *ChordBuilder) Note(tone int, beats float64) *ChordBuilder {
	scale := cb.track.score.Scale
	return cb.With(scale.Note(tone), beats)
}

func (cb *ChordBuilder) Flat(tone int, beats float64) *ChordBuilder {
	scale := cb.track.score.Scale
	return cb.With(scale.Flat(tone), beats)
}

func (cb *ChordBuilder) Sharp(tone int, beats float64) *ChordBuilder {
	scale := cb.track.score.Scale
	return cb.With(scale.Sharp(tone), beats)
}

func (cb *ChordBuilder) Done(entireBeats float64) {
	track := cb.track
	for _, note := range cb.notes {
		track.notes = append(track.notes, note)
	}
	if entireBeats < 0 {
		cb.track.offset += cb.duration
	} else {
		bpm := cb.track.score.Bpm
		cb.track.offset += entireBeats / (bpm / 60.0)
	}
}

func (cb *ChordBuilder) Ok() {
	cb.Done(-1)
}
