//
// Created by kaede on 2024/12/08.
//

#pragma once
#include <variant>

#include "MIDIInputPin.h"
#include "MIDIOutputPin.h"

namespace logica::synth::node::pin {

using InputPin = std::variant<
  MIDIInputPin
>;

using OutputPin = std::variant<
  MIDIOutputPin
>;

enum class ValueType {
  MIDI,
};

}
