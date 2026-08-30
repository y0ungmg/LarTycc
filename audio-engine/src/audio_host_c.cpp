#include "lartycc/audio_host_c.h"

#include "lartycc/audio_device.hpp"
#include "lartycc/audio_engine.hpp"

#include <algorithm>
#include <cstring>
#include <new>
#include <span>
#include <string>
#include <vector>

struct lartycc_audio_host final {
  lartycc::audio::AudioEngine engine;
  lartycc::audio::AudioOutput output{engine};
  std::vector<lartycc::audio::AudioDeviceInfo> devices;
};

namespace {

bool copy_string(const std::string& source, char* destination, const size_t capacity) noexcept {
  if (destination == nullptr || capacity <= source.size()) return false;
  std::memcpy(destination, source.data(), source.size());
  destination[source.size()] = '\0';
  return true;
}

}  // namespace

extern "C" {

lartycc_audio_host* lartycc_audio_host_create(void) {
  try {
    return new lartycc_audio_host{};
  } catch (...) {
    return nullptr;
  }
}

void lartycc_audio_host_destroy(lartycc_audio_host* host) { delete host; }

bool lartycc_audio_host_available(const lartycc_audio_host* host) {
  return host != nullptr && host->output.available();
}

size_t lartycc_audio_host_refresh_devices(lartycc_audio_host* host) {
  if (host == nullptr) return 0;
  try {
    host->devices = host->output.devices();
    return host->devices.size();
  } catch (...) {
    host->devices.clear();
    return 0;
  }
}

bool lartycc_audio_host_device_info(const lartycc_audio_host* host, const size_t index,
                                    char* id, const size_t id_capacity,
                                    char* name, const size_t name_capacity,
                                    bool* is_default) {
  if (host == nullptr || index >= host->devices.size() || is_default == nullptr) return false;
  const auto& device = host->devices[index];
  if (!copy_string(device.id, id, id_capacity) ||
      !copy_string(device.name, name, name_capacity)) {
    return false;
  }
  *is_default = device.is_default;
  return true;
}

bool lartycc_audio_host_load_mono(lartycc_audio_host* host,
                                  const float* samples, const size_t sample_count) {
  if (host == nullptr || host->output.is_running() || samples == nullptr || sample_count == 0) {
    return false;
  }
  try {
    return host->engine.load_mono_sample(std::span{samples, sample_count});
  } catch (...) {
    return false;
  }
}

bool lartycc_audio_host_start(lartycc_audio_host* host, const char* device_id,
                              const unsigned int sample_rate,
                              const unsigned int period_frames) {
  if (host == nullptr || sample_rate == 0 || period_frames == 0) return false;
  try {
    lartycc::audio::AudioDeviceConfig config;
    if (device_id != nullptr) config.device_id = device_id;
    config.sample_rate = sample_rate;
    config.period_frames = period_frames;
    return host->output.start(config);
  } catch (...) {
    return false;
  }
}

void lartycc_audio_host_stop(lartycc_audio_host* host) {
  if (host == nullptr) return;
  host->engine.stop();
  host->output.stop();
}

bool lartycc_audio_host_play(lartycc_audio_host* host) {
  return host != nullptr && host->output.is_running() && host->engine.play();
}

bool lartycc_audio_host_seek(lartycc_audio_host* host, const size_t frame) {
  return host != nullptr && host->engine.seek(frame);
}

void lartycc_audio_host_set_gain(lartycc_audio_host* host, const float gain) {
  if (host != nullptr) host->engine.set_master_gain(gain);
}

bool lartycc_audio_host_is_playing(const lartycc_audio_host* host) {
  return host != nullptr && host->engine.is_playing();
}

size_t lartycc_audio_host_position(const lartycc_audio_host* host) {
  return host == nullptr ? 0 : host->engine.position();
}

size_t lartycc_audio_host_callback_count(const lartycc_audio_host* host) {
  return host == nullptr ? 0 : host->output.callback_count();
}

}  // extern "C"
