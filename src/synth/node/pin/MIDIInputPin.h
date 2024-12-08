//
// Created by kaede on 2024/12/08.
//

#pragma once
#include "PinBase.h"

namespace logica::synth::node::pin {

class MIDIInputPin : public InputPinBase {
public:
  MIDIInputPin()
  : InputPinBase(
    ValueType::MIDI
  )
  {}
};

}
