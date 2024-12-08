//
// Created by kaede on 2024/12/08.
//

#pragma once
#include <string>

#include "Pin.h"

namespace logica::synth::node::pin {

class PinBase {
protected:
  ~PinBase() = default;

public:
  PinBase() = delete;
  PinBase(const PinBase&) = delete;
  PinBase(PinBase&&) = delete;
  PinBase& operator=(const PinBase&) = delete;
  PinBase& operator=(PinBase&&) = delete;

protected:
  explicit PinBase(
    ValueType const valueType
  )
  : valueType_(valueType) {
  }

protected:
  std::string const name_;
  ValueType const valueType_;
public:
  [[nodiscard]] std::string const& name() const { return this->name_; }
  [[nodiscard]] ValueType const& valueType() const { return this->valueType_; }
};

class InputPinBase : public PinBase {
protected:
  explicit InputPinBase(ValueType const valueType)
  : PinBase(valueType)
  {}

public:
  InputPinBase() = delete;
  InputPinBase(const InputPinBase&) = delete;
  InputPinBase(InputPinBase&&) = delete;
  InputPinBase& operator=(const InputPinBase&) = delete;
  InputPinBase& operator=(InputPinBase&&) = delete;
};

class OutputPinBase : public PinBase {
protected:
  explicit OutputPinBase(ValueType const valueType)
  : PinBase(valueType)
  {}

public:
  OutputPinBase() = delete;
  OutputPinBase(const OutputPinBase&) = delete;
  OutputPinBase(OutputPinBase&&) = delete;
  OutputPinBase& operator=(const OutputPinBase&) = delete;
  OutputPinBase& operator=(OutputPinBase&&) = delete;
};

}
