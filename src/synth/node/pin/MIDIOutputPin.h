//
// Created by kaede on 2024/12/08.
//

#pragma once
#include "PinBase.h"

namespace logica::synth::node::pin {

class MIDIOutputPin : public OutputPinBase {
public:
  MIDIOutputPin()
  : OutputPinBase(
    ValueType::MIDI
    )
  {}
};

}
