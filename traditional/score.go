package traditional

import "sort"

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

func (track *ScoreTrack) AddNote(deg int, duration float64) *Note {
	score := track.score
	bpm := score.Bpm
	note := score.Scale.MakeNote(deg)
	note.Offset = track.offset
	note.Duration = duration / bpm
	track.offset += note.Duration
	note.SeekOffset = 0
	track.notes = append(track.notes, note)
	return note
}

func (track *ScoreTrack) AddRest(duration float64) {
	bpm := track.score.Bpm
	track.offset += duration / bpm
}

func (track *ScoreTrack) End() {
	score := track.score
	for _, pos := range track.notes {
		score.Notes = append(score.Notes, pos)
	}
	track.score = nil
}
