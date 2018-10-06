// +build darwin freebsd

package speaker

// #cgo darwin  LDFLAGS: -framework OpenAL
// #cgo freebsd LDFLAGS: -lopenal
//
// #ifdef __APPLE__
// #include <OpenAL/al.h>
// #include <OpenAL/alc.h>
// #else
// #include <AL/al.h>
// #include <AL/alc.h>
// #endif
import "C"

import (
	"errors"
	"fmt"
)

type jack struct {
	numChannels int
	sampleRates int
	device      *C.ALCdevice
	context     *C.ALCcontext
}

func CreateJack(numChan, sampleRate, buffSize int) (Jack, error) {
	var err error
	jack := &jack{
		numChannels: numChan,
		sampleRates: sampleRate,
	}
	jack.device = C.alcOpenDevice(nil) // const ALCchar *devicename
	err = checkError()
	if err != nil {
		return nil, fmt.Errorf("unable to open OpenAL device: %v", err)
	}
	jack.context = C.alcCreateContext(
		jack.device, // ALCdevice *device
		nil)         // const ALCint* attrlist
	if err = jack.checkError(); err != nil {
		return nil, fmt.Errorf("unable to create context: %v", err)
	}
	return jack, nil
}

func (jack *jack) PlugIn(cable Line) {

}

func (jack *jack) Start() error {
	var err error
	C.alcMakeContextCurrent(jack.context)
	if err = jack.checkError(); err != nil {
		return fmt.Errorf("unable to make context current: %v", err)
	}

	return nil
}

func (jack *jack) Stop() {

}
func (jack *jack) Close() {
	C.alcDestroyContext(jack.context)
	C.alcCloseDevice(jack.device)
}

func (jack *jack) Channels() int {
	return jack.numChannels
}

func (jack *jack) SampleRates() int {
	return jack.sampleRates
}

func checkError() error {
	cerr := C.alGetError()
	if cerr == C.AL_NO_ERROR {
		return nil
	}
	return errors.New(C.GoString(C.alGetString(cerr)))
}

func (jack *jack) checkError() error {
	cerr := C.alcGetError(jack.device)
	if cerr == C.AL_NO_ERROR {
		return nil
	}
	return errors.New(C.GoString(C.alGetString(cerr)))
}
