package traditional

import (
	"math"
	"sort"
)

// Basic types
type Bpm float64
type Beats float64
type Offset float64   /* in sec */
type Duration float64 /* in sec */
type Freq float64     /* in hz */
type Tone int

func (off *Offset) IncreaseBeats(beats Beats, bpm Bpm) {
	off.Increase(beats2sec(beats, bpm))
}

func (off *Offset) Increase(d Duration) {
	*((*float64)(off)) += float64(d)
}

type Note struct {
	Offset     Offset
	Freq       Freq
	Beats      Beats
	SeekOffset Offset
}

type Scale interface {
	Note(deg Tone) *Note
	Flat(deg Tone) *Note
	Sharp(deg Tone) *Note
}

type Score struct {
	Bpm   Bpm
	Scale Scale
	Notes []*Note
}

func NewScore(bpm float64, scale Scale) *Score {
	return &Score{
		Bpm:   Bpm(bpm),
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
		offset: Offset(offset),
		notes:  make([]*Note, 0),
	}
}

func (score *Score) CalcDuration(beats Beats) Duration {
	return beats2sec(beats, score.Bpm)
}

func beats2sec(beats Beats, bpm Bpm) Duration {
	return Duration(float64(beats) / (float64(bpm) / 60.0))
}

// ----------------------------------------------------------------------------

type ScoreTrack struct {
	score  *Score
	offset Offset
	notes  []*Note
}

func (track *ScoreTrack) With(note *Note, beats Beats) *ChordBuilder {
	cb := &ChordBuilder{
		track:  track,
		offset: track.offset,
		notes:  make([]*Note, 0),
	}
	return cb.With(note, beats)
}

func (track *ScoreTrack) Note(tone Tone, beats Beats) *ChordBuilder {
	return track.With(track.score.Scale.Note(tone), beats)
}

func (track *ScoreTrack) Flat(tone Tone, beats Beats) *ChordBuilder {
	return track.With(track.score.Scale.Flat(tone), beats)
}

func (track *ScoreTrack) Sharp(tone Tone, beats Beats) *ChordBuilder {
	return track.With(track.score.Scale.Sharp(tone), beats)
}

func (track *ScoreTrack) Rest(beats Beats) {
	bpm := track.score.Bpm
	track.offset.IncreaseBeats(beats, bpm)
}

func (track *ScoreTrack) Close() {
	score := track.score
	for _, pos := range track.notes {
		score.Notes = append(score.Notes, pos)
	}
	// Closed.
	track.score = nil
	track.offset = Offset(math.NaN())
	track.score = nil
}

// ----------------------------------------------------------------------------

type ChordBuilder struct {
	track  *ScoreTrack
	offset Offset
	beats  Beats
	notes  []*Note
}

func (cb *ChordBuilder) With(note *Note, beats Beats) *ChordBuilder {
	note.Offset = cb.offset
	note.Beats = beats
	note.SeekOffset = 0
	cb.notes = append(cb.notes, note)

	if note.Beats > cb.beats {
		cb.beats = note.Beats
	}

	return cb
}

func (cb *ChordBuilder) Note(tone Tone, beats Beats) *ChordBuilder {
	scale := cb.track.score.Scale
	return cb.With(scale.Note(tone), beats)
}

func (cb *ChordBuilder) Flat(tone Tone, beats Beats) *ChordBuilder {
	scale := cb.track.score.Scale
	return cb.With(scale.Flat(tone), beats)
}

func (cb *ChordBuilder) Sharp(tone Tone, beats Beats) *ChordBuilder {
	scale := cb.track.score.Scale
	return cb.With(scale.Sharp(tone), beats)
}

func (cb *ChordBuilder) Done(entireBeats Beats) {
	track := cb.track
	for _, note := range cb.notes {
		track.notes = append(track.notes, note)
	}
	bpm := cb.track.score.Bpm
	if entireBeats < 0 {
		cb.track.offset.IncreaseBeats(cb.beats, bpm)
	} else {
		cb.track.offset.IncreaseBeats(entireBeats, bpm)
	}
}

func (cb *ChordBuilder) Ok() {
	cb.Done(-1)
}
