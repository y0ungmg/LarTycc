#include "lartycc/audio_device.hpp"

#include "lartycc/audio_engine.hpp"

#include <algorithm>
#include <array>
#include <atomic>
#include <chrono>
#include <cstddef>
#include <cstring>
#include <span>
#include <string_view>
#include <utility>

#include <miniaudio.h>

namespace lartycc::audio {
namespace {

static_assert(std::atomic<std::uint64_t>::is_always_lock_free);

std::uint64_t monotonic_nanoseconds() noexcept {
  const auto elapsed = std::chrono::steady_clock::now().time_since_epoch();
  return static_cast<std::uint64_t>(
      std::chrono::duration_cast<std::chrono::nanoseconds>(elapsed).count());
}

std::string encode_id(const ma_device_id& id) {
  constexpr auto digits = std::string_view{"0123456789abcdef"};
  const auto* bytes = reinterpret_cast<const unsigned char*>(&id);
  std::string result(sizeof(id) * 2U, '0');
  for (std::size_t index = 0; index < sizeof(id); ++index) {
    result[index * 2U] = digits[bytes[index] >> 4U];
    result[index * 2U + 1U] = digits[bytes[index] & 0x0fU];
  }
  return result;
}

}  // namespace

class AudioOutput::Impl final {
 public:
  explicit Impl(AudioEngine& engine) : engine_(engine) {
    context_ready_ = ma_context_init(nullptr, 0, nullptr, &context_) == MA_SUCCESS;
  }

  ~Impl() {
    stop();
    if (context_ready_) ma_context_uninit(&context_);
  }

  [[nodiscard]] std::vector<AudioDeviceInfo> devices() const {
    if (!context_ready_) return {};
    ma_device_info* playback = nullptr;
    ma_uint32 playback_count = 0;
    if (ma_context_get_devices(const_cast<ma_context*>(&context_), &playback, &playback_count,
                               nullptr, nullptr) != MA_SUCCESS) {
      return {};
    }
    std::vector<AudioDeviceInfo> result;
    result.reserve(playback_count);
    for (ma_uint32 index = 0; index < playback_count; ++index) {
      result.push_back({encode_id(playback[index].id), playback[index].name,
                        playback[index].isDefault == MA_TRUE});
    }
    return result;
  }

  [[nodiscard]] bool start(const AudioDeviceConfig& requested) {
    stop();
    if (!context_ready_ || requested.channels == 0 || requested.period_frames == 0 ||
        !engine_.prepare(requested.sample_rate, requested.period_frames)) {
      return false;
    }

    auto config = ma_device_config_init(ma_device_type_playback);
    config.playback.format = ma_format_f32;
    config.playback.channels = requested.channels;
    config.sampleRate = requested.sample_rate;
    config.periodSizeInFrames = requested.period_frames;
    config.performanceProfile = ma_performance_profile_low_latency;
    config.dataCallback = data_callback;
    config.pUserData = this;

    std::array<ma_device_id, 1> selected{};
    if (!requested.device_id.empty()) {
      ma_device_info* playback = nullptr;
      ma_uint32 playback_count = 0;
      if (ma_context_get_devices(&context_, &playback, &playback_count, nullptr, nullptr) !=
          MA_SUCCESS) {
        return false;
      }
      const auto match = std::find_if(playback, playback + playback_count, [&](const auto& info) {
        return encode_id(info.id) == requested.device_id;
      });
      if (match == playback + playback_count) return false;
      selected[0] = match->id;
      config.playback.pDeviceID = selected.data();
    }

    channels_ = requested.channels;
    sample_rate_ = requested.sample_rate;
    measure_timing_ = requested.measure_timing;
    callback_count_.store(0, std::memory_order_relaxed);
    timing_metrics_.reset();
    if (ma_device_init(&context_, &config, &device_) != MA_SUCCESS) return false;
    device_ready_ = true;
    if (ma_device_start(&device_) != MA_SUCCESS) {
      stop();
      return false;
    }
    running_.store(true, std::memory_order_release);
    return true;
  }

  void stop() noexcept {
    running_.store(false, std::memory_order_release);
    if (device_ready_) {
      ma_device_stop(&device_);
      ma_device_uninit(&device_);
      device_ready_ = false;
    }
  }

  static void data_callback(ma_device* device, void* output, const void*, ma_uint32 frames) {
    auto* self = static_cast<Impl*>(device->pUserData);
    const auto started = self->measure_timing_ ? monotonic_nanoseconds() : 0;
    auto samples = std::span{static_cast<float*>(output),
                             static_cast<std::size_t>(frames) * self->channels_};
    self->engine_.process({static_cast<double>(self->sample_rate_), frames, self->channels_},
                          samples);
    self->callback_count_.fetch_add(1, std::memory_order_relaxed);
    if (self->measure_timing_) {
      self->timing_metrics_.record(started, monotonic_nanoseconds(), frames,
                                   self->sample_rate_);
    }
  }

  AudioEngine& engine_;
  ma_context context_{};
  ma_device device_{};
  bool context_ready_{false};
  bool device_ready_{false};
  unsigned int channels_{2};
  unsigned int sample_rate_{48'000};
  std::atomic<bool> running_{false};
  std::atomic<std::size_t> callback_count_{0};
  bool measure_timing_{false};
  RealtimeMetrics timing_metrics_;
};

AudioOutput::AudioOutput(AudioEngine& engine) : impl_(std::make_unique<Impl>(engine)) {}
AudioOutput::~AudioOutput() = default;
AudioOutput::AudioOutput(AudioOutput&&) noexcept = default;
AudioOutput& AudioOutput::operator=(AudioOutput&&) noexcept = default;
bool AudioOutput::available() const noexcept { return impl_ && impl_->context_ready_; }
std::vector<AudioDeviceInfo> AudioOutput::devices() const { return impl_->devices(); }
bool AudioOutput::start(const AudioDeviceConfig& config) { return impl_->start(config); }
void AudioOutput::stop() noexcept { impl_->stop(); }
bool AudioOutput::is_running() const noexcept {
  return impl_->running_.load(std::memory_order_acquire);
}
std::size_t AudioOutput::callback_count() const noexcept {
  return impl_->callback_count_.load(std::memory_order_relaxed);
}
RealtimeMetricsSnapshot AudioOutput::timing_metrics() const noexcept {
  return impl_->timing_metrics_.snapshot();
}

}  // namespace lartycc::audio
