#pragma once

#include <cstddef>
#include <span>

namespace lartycc::audio {

struct ProcessContext {
  double sample_rate;
  std::size_t frames;
};

class AudioEngine final {
 public:
  [[nodiscard]] bool prepare(double sample_rate, std::size_t max_block_size) noexcept;
  void process(ProcessContext context, std::span<float> interleaved_output) noexcept;
  [[nodiscard]] bool is_prepared() const noexcept;

 private:
  double sample_rate_{0.0};
  std::size_t max_block_size_{0};
};

}  // namespace lartycc::audio

