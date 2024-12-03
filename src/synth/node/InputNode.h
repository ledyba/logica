//
// Created by kaede on 2024/11/01.
//

#pragma once

#include "NodeBase.h"

namespace logica::synth::node {

class InputNode : public NodeBase {
public:
  explicit InputNode(Context& ctx);
};

}
