//
// Created by kaede on 2024/11/01.
//

#pragma once

#include "Context.h"

namespace logica::synth::node {

// node base class.
class NodeBase {
public:
  explicit NodeBase() = delete;
public:
protected:
  int id_;
protected:
  explicit NodeBase(Context& ctx): id_(ctx.next()){}
  ~NodeBase() = default;
};

}
