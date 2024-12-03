//
// Created by kaede on 2024/11/01.
//

#pragma once

namespace logica::synth::node {

enum SignalType {
  MidiNote, // 0 ～ 127, https://www.asahi-net.or.jp/~hb9t-ktd/music/Japan/Research/DTM/freq_map.html
  Frequency, // A4 = 440.0Hz
  Raw, // -1.0～1.0, Audio signal is "raw".
};

}
