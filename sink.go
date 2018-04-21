package logica

type Sink interface {
	Play(stream Stream, offset, duration float64)
	Close()
}
