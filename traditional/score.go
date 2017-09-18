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
	MakeNote(deg int) *Note
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

type ScoreTrack struct {
	score  *Score
	offset float64 /* in sec */
	notes  []*Note
}

func (track *ScoreTrack) AddNote(beats float64, tone int) *Note {
	score := track.score
	bpm := score.Bpm
	note := score.Scale.MakeNote(tone)
	note.Offset = track.offset
	note.Duration = beats / (bpm / 60.0)
	track.offset += note.Duration
	note.SeekOffset = 0
	track.notes = append(track.notes, note)
	return note
}
func (track *ScoreTrack) Fork(n int, f func(subtracks []*ScoreTrack)) {
	subs := make([]*ScoreTrack, n)
	for i := range subs {
		subs[i] = &ScoreTrack{
			score:  track.score,
			offset: track.offset,
			notes:  make([]*Note, 0),
		}
	}
	f(subs)
	for _, sub := range subs {
		for _, n := range sub.notes {
			track.notes = append(track.notes, n)
		}
		if track.offset < sub.offset {
			track.offset = sub.offset
		}
	}
}
func (track *ScoreTrack) AddNotes(beats float64, tones ...int) []*Note {
	score := track.score
	bpm := score.Bpm
	notes := make([]*Note, len(tones))
	duration := beats / (bpm / 60.0)
	for i := range notes {
		note := score.Scale.MakeNote(tones[i])
		note.Offset = track.offset
		note.Duration = duration
		note.SeekOffset = 0
		track.notes = append(track.notes, note)
		notes[i] = note
	}
	track.offset += duration
	return notes
}

func (track *ScoreTrack) AddRest(beats float64) {
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
