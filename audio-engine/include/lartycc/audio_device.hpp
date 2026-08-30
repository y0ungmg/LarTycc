#pragma once

#include <cstddef>
#include <memory>
#include <string>
#include <vector>

namespace lartycc::audio {

class AudioEngine;

struct AudioDeviceInfo {
  std::string id;
  std::string name;
  bool is_default{false};
};

struct AudioDeviceConfig {
  std::string device_id;
  unsigned int sample_rate{48'000};
  unsigned int channels{2};
  unsigned int period_frames{128};
};

class AudioOutput final {
 public:
  explicit AudioOutput(AudioEngine& engine);
  ~AudioOutput();
  AudioOutput(const AudioOutput&) = delete;
  AudioOutput& operator=(const AudioOutput&) = delete;
  AudioOutput(AudioOutput&&) noexcept;
  AudioOutput& operator=(AudioOutput&&) noexcept;

  [[nodiscard]] bool available() const noexcept;
  [[nodiscard]] std::vector<AudioDeviceInfo> devices() const;
  [[nodiscard]] bool start(const AudioDeviceConfig& config);
  void stop() noexcept;
  [[nodiscard]] bool is_running() const noexcept;
  [[nodiscard]] std::size_t callback_count() const noexcept;

 private:
  class Impl;
  std::unique_ptr<Impl> impl_;
};

}  // namespace lartycc::audio
