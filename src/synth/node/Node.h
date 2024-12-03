//
// Created by kaede on 2024/11/01.
//

#pragma once

#include <variant>

// Nodes
#include "InputNode.h"

namespace logica::synth::node {

using Node = std::variant<
  InputNode
>;

}
