#include "lartycc/audio_engine.hpp"

#include <algorithm>
#include <cmath>

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

bool AudioEngine::load_mono_sample(const std::span<const float> samples) {
  if (is_playing() || samples.empty()) {
    return false;
  }
  sample_.assign(samples.begin(), samples.end());
  position_.store(0, std::memory_order_release);
  return true;
}

bool AudioEngine::play() noexcept {
  if (!is_prepared() || sample_.empty()) {
    return false;
  }
  if (position() >= sample_.size()) {
    position_.store(0, std::memory_order_release);
  }
  playing_.store(true, std::memory_order_release);
  return true;
}

void AudioEngine::stop() noexcept {
  playing_.store(false, std::memory_order_release);
}

bool AudioEngine::seek(const std::size_t frame) noexcept {
  if (frame > sample_.size()) {
    return false;
  }
  position_.store(frame, std::memory_order_release);
  return true;
}

void AudioEngine::set_master_gain(const float gain) noexcept {
  const auto safe_gain = std::isfinite(gain) ? std::clamp(gain, 0.0F, 2.0F) : 0.0F;
  master_gain_.store(safe_gain, std::memory_order_release);
}

void AudioEngine::process(const ProcessContext context,
                          std::span<float> interleaved_output) noexcept {
  std::fill(interleaved_output.begin(), interleaved_output.end(), 0.0F);
  if (!is_prepared() || context.sample_rate != sample_rate_ || context.frames > max_block_size_ ||
      context.output_channels == 0 ||
      interleaved_output.size() < context.frames * context.output_channels || !is_playing()) {
    return;
  }

  auto frame = position_.load(std::memory_order_acquire);
  const auto gain = master_gain_.load(std::memory_order_acquire);
  for (std::size_t i = 0; i < context.frames && frame < sample_.size(); ++i, ++frame) {
    const auto value = sample_[frame] * gain;
    for (std::size_t channel = 0; channel < context.output_channels; ++channel) {
      interleaved_output[i * context.output_channels + channel] = value;
    }
  }
  position_.store(frame, std::memory_order_release);
  if (frame >= sample_.size()) {
    playing_.store(false, std::memory_order_release);
  }
}

bool AudioEngine::is_prepared() const noexcept {
  return sample_rate_ > 0.0 && max_block_size_ > 0;
}

bool AudioEngine::is_playing() const noexcept {
  return playing_.load(std::memory_order_acquire);
}

std::size_t AudioEngine::position() const noexcept {
  return position_.load(std::memory_order_acquire);
}

std::size_t AudioEngine::sample_length() const noexcept {
  return sample_.size();
}

}  // namespace lartycc::audio
