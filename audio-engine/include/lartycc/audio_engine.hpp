#pragma once

#include <cstddef>
#include <atomic>
#include <span>
#include <vector>

namespace lartycc::audio {

struct ProcessContext {
  double sample_rate;
  std::size_t frames;
  std::size_t output_channels{2};
};

class AudioEngine final {
 public:
  [[nodiscard]] bool prepare(double sample_rate, std::size_t max_block_size) noexcept;
  [[nodiscard]] bool load_mono_sample(std::span<const float> samples);
  [[nodiscard]] bool play() noexcept;
  void stop() noexcept;
  [[nodiscard]] bool seek(std::size_t frame) noexcept;
  void set_master_gain(float gain) noexcept;
  void process(ProcessContext context, std::span<float> interleaved_output) noexcept;
  [[nodiscard]] bool is_prepared() const noexcept;
  [[nodiscard]] bool is_playing() const noexcept;
  [[nodiscard]] std::size_t position() const noexcept;
  [[nodiscard]] std::size_t sample_length() const noexcept;

 private:
  double sample_rate_{0.0};
  std::size_t max_block_size_{0};
  std::vector<float> sample_;
  std::atomic<std::size_t> position_{0};
  std::atomic<float> master_gain_{1.0F};
  std::atomic<bool> playing_{false};
};

}  // namespace lartycc::audio
