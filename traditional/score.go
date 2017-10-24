package traditional

import (
	"math"
	"sort"
)

// Basic types
type Bpm float64
type Beats float64
type Offset float64 /* in beats */
type Freq float64   /* in hz */
type Tone int

func (off *Offset) Increase(beats Beats) {
	*((*float64)(off)) += float64(beats)
}

func (off Offset) Add(beats Beats) Offset {
	return Offset(float64(off) + float64(beats))
}

func (f Freq) AsFloat() float64 {
	return float64(f)
}

type Note struct {
	Offset     Offset
	Freq       Freq
	Beats      Beats
	SeekOffset Offset
}

type Scale interface {
	Note(tone Tone) *Note
	Flat(tone Tone) *Note
	Sharp(tone Tone) *Note
	Raw(deg int) *Note
}

type Score struct {
	Bpm    Bpm
	Scale  Scale
	Tracks []*Track
}

func NewScore(bpm float64, scale Scale) *Score {
	return &Score{
		Bpm:    Bpm(bpm),
		Scale:  scale,
		Tracks: make([]*Track, 0),
	}
}

func (score *Score) NewTrack(offset Offset) *Track {
	return &Track{
		score:  score,
		offset: offset,
		Notes:  make([]*Note, 0),
	}
}

func (score *Score) CalcDuration(beats Beats) float64 {
	return beats2sec(beats, score.Bpm)
}

func (score *Score) CalcOffset(off Offset) float64 {
	return beats2sec(Beats(off), score.Bpm)
}

func beats2sec(beats Beats, bpm Bpm) float64 {
	return float64(beats) / (float64(bpm) / 60.0)
}

// ----------------------------------------------------------------------------

type Track struct {
	score  *Score
	offset Offset
	Notes  []*Note
}

func (track *Track) scoreSorter(i, j int) bool {
	return track.Notes[i].Offset < track.Notes[j].Offset
}

func (track *Track) Sort() {
	sort.Slice(track.Notes, track.scoreSorter)
}

func (track *Track) With(note *Note, beats Beats) *ChordBuilder {
	cb := &ChordBuilder{
		track:      track,
		baseOffset: track.offset,
	}
	return cb.With(note, beats)
}

func (track *Track) Note(tone Tone, beats Beats) *ChordBuilder {
	return track.With(track.score.Scale.Note(tone), beats)
}

func (track *Track) Flat(tone Tone, beats Beats) *ChordBuilder {
	return track.With(track.score.Scale.Flat(tone), beats)
}

func (track *Track) Sharp(tone Tone, beats Beats) *ChordBuilder {
	return track.With(track.score.Scale.Sharp(tone), beats)
}

func (track *Track) Raw(deg int, beats Beats) *ChordBuilder {
	return track.With(track.score.Scale.Raw(deg), beats)
}

func (track *Track) Rest(beats Beats) {
	track.offset.Increase(beats)
}

func (track *Track) Seek(offset Offset) {
	track.offset = offset
}

func (track *Track) Close() {
	score := track.score
	score.Tracks = append(score.Tracks, track)
	// Closed.
	track.score = nil
	track.offset = Offset(math.NaN())
}

// ----------------------------------------------------------------------------

type ChordBuilder struct {
	track      *Track
	baseOffset Offset
	beats      Beats
}

func (cb *ChordBuilder) With(note *Note, beats Beats) *ChordBuilder {
	note.Offset = cb.baseOffset
	note.Beats = beats
	note.SeekOffset = 0

	track := cb.track
	track.Notes = append(track.Notes, note)

	if note.Beats > cb.beats {
		cb.beats = note.Beats
	}

	nextOffset := cb.baseOffset.Add(note.Beats)

	// Longest
	if nextOffset > track.offset {
		track.offset = nextOffset
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

func (cb *ChordBuilder) Raw(deg int, beats Beats) *ChordBuilder {
	scale := cb.track.score.Scale
	return cb.With(scale.Raw(deg), beats)
}

func (cb *ChordBuilder) Done(entireBeats Beats) {
	cb.track.offset = cb.baseOffset.Add(entireBeats)
}
