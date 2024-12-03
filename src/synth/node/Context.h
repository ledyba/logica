//
// Created by kaede on 2024/11/01.
//

#pragma once

namespace logica::synth::node {

class Context final {
public:
  Context() = default;
  ~Context() = default;
public:
  int next() {
    int id = currentId_;
    currentId_++;
    return id;
  }
private:
  int currentId_ = 1;
};

}
