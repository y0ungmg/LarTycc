#include "lartycc/audio_engine.hpp"

#include <algorithm>

namespace lartycc::audio {

bool AudioEngine::prepare(const double sample_rate,
                          const std::size_t max_block_size) noexcept {
  if (sample_rate <= 0.0 || max_block_size == 0) {
    return false;
  }
  sample_rate_ = sample_rate;
  max_block_size_ = max_block_size;
  return true;
}

void AudioEngine::process(const ProcessContext context,
                          std::span<float> interleaved_output) noexcept {
  if (!is_prepared() || context.sample_rate != sample_rate_ ||
      context.frames > max_block_size_) {
    return;
  }
  std::fill(interleaved_output.begin(), interleaved_output.end(), 0.0F);
}

bool AudioEngine::is_prepared() const noexcept {
  return sample_rate_ > 0.0 && max_block_size_ > 0;
}

}  // namespace lartycc::audio

