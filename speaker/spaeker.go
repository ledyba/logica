package speaker

type DataFormat int

type Jack interface {
	PlugIn(cable Line)

	Start() error
	Stop()
	Close()

	// info
	Channels() int
	SampleRates() int
}

type Line interface {
	FullFill(speaker Line, buff []float64) error
}
