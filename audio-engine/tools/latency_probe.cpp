#include "lartycc/audio_device.hpp"
#include "lartycc/audio_engine.hpp"

#include <algorithm>
#include <charconv>
#include <chrono>
#include <cmath>
#include <cstdint>
#include <iomanip>
#include <iostream>
#include <numbers>
#include <optional>
#include <string>
#include <string_view>
#include <thread>
#include <vector>

namespace {

struct Options {
  std::string device_id;
  unsigned int seconds{30};
  unsigned int sample_rate{48'000};
  unsigned int period_frames{128};
  std::uint64_t max_deadline_misses{0};
};

std::optional<unsigned int> parse_unsigned(const std::string_view value) {
  unsigned int parsed = 0;
  const auto [end, error] = std::from_chars(value.data(), value.data() + value.size(), parsed);
  if (error != std::errc{} || end != value.data() + value.size()) return std::nullopt;
  return parsed;
}

bool parse_options(const int argc, char** argv, Options& options) {
  for (int index = 1; index < argc; ++index) {
    const std::string_view argument{argv[index]};
    if (index + 1 >= argc) return false;
    const std::string_view value{argv[++index]};
    if (argument == "--device") {
      options.device_id = value;
    } else if (argument == "--seconds") {
      const auto parsed = parse_unsigned(value);
      if (!parsed || *parsed == 0 || *parsed > 600) return false;
      options.seconds = *parsed;
    } else if (argument == "--sample-rate") {
      const auto parsed = parse_unsigned(value);
      if (!parsed || *parsed < 8'000 || *parsed > 384'000) return false;
      options.sample_rate = *parsed;
    } else if (argument == "--period-frames") {
      const auto parsed = parse_unsigned(value);
      if (!parsed || *parsed < 16 || *parsed > 8'192) return false;
      options.period_frames = *parsed;
    } else if (argument == "--max-deadline-misses") {
      const auto parsed = parse_unsigned(value);
      if (!parsed) return false;
      options.max_deadline_misses = *parsed;
    } else {
      return false;
    }
  }
  return true;
}

std::string json_escape(const std::string_view input) {
  std::string output;
  output.reserve(input.size());
  for (const char character : input) {
    switch (character) {
      case '"': output += "\\\""; break;
      case '\\': output += "\\\\"; break;
      case '\n': output += "\\n"; break;
      case '\r': output += "\\r"; break;
      case '\t': output += "\\t"; break;
      default: output += character; break;
    }
  }
  return output;
}

const lartycc::audio::AudioDeviceInfo* selected_device(
    const std::vector<lartycc::audio::AudioDeviceInfo>& devices,
    const std::string_view requested_id) {
  const auto selected = std::find_if(devices.begin(), devices.end(), [&](const auto& device) {
    return requested_id.empty() ? device.is_default : device.id == requested_id;
  });
  return selected == devices.end() ? nullptr : &*selected;
}

}  // namespace

int main(const int argc, char** argv) {
  Options options;
  if (!parse_options(argc, argv, options)) {
    std::cerr << "usage: lartycc_latency_probe [--device ID] [--seconds 1..600] "
                 "[--sample-rate HZ] [--period-frames 16..8192] "
                 "[--max-deadline-misses COUNT]\n";
    return 64;
  }

  lartycc::audio::AudioEngine engine;
  if (!engine.prepare(options.sample_rate, options.period_frames)) return 1;
  std::vector<float> sample(static_cast<std::size_t>(options.sample_rate) * options.seconds);
  for (std::size_t frame = 0; frame < sample.size(); ++frame) {
    const auto time = static_cast<double>(frame) / options.sample_rate;
    sample[frame] = static_cast<float>(std::sin(2.0 * std::numbers::pi * 220.0 * time) * 0.1);
  }
  if (!engine.load_mono_sample(sample) || !engine.play()) return 1;

  lartycc::audio::AudioOutput output(engine);
  const auto devices = output.devices();
  const lartycc::audio::AudioDeviceConfig config{
      options.device_id, options.sample_rate, 2, options.period_frames, true};
  if (!output.start(config)) {
    std::cerr << "failed to start the requested playback device\n";
    return 1;
  }
  std::this_thread::sleep_for(std::chrono::seconds(options.seconds));
  output.stop();

  const auto metrics = output.timing_metrics();
  const auto* selected = selected_device(devices, options.device_id);
  std::string_view device_id{options.device_id};
  std::string_view device_name{"system default"};
  if (selected != nullptr) {
    device_id = selected->id;
    device_name = selected->name;
  }
  const auto expected_callbacks =
      static_cast<std::uint64_t>(options.seconds) * options.sample_rate / options.period_frames;
  const auto nominal_period_ms =
      1'000.0 * options.period_frames / static_cast<double>(options.sample_rate);
  const auto enough_callbacks = metrics.callback_count >= expected_callbacks * 9U / 10U;
  const auto process_within_period =
      metrics.max_process_time_ns <=
      static_cast<std::uint64_t>(nominal_period_ms * 1'000'000.0 * 0.7);
  const auto passed = enough_callbacks && process_within_period &&
                      metrics.deadline_miss_count <= options.max_deadline_misses;

  std::cout << std::fixed << std::setprecision(6)
            << "{\n"
            << "  \"schema_version\": 1,\n"
            << "  \"device_id\": \"" << json_escape(device_id) << "\",\n"
            << "  \"device_name\": \"" << json_escape(device_name) << "\",\n"
            << "  \"sample_rate\": " << options.sample_rate << ",\n"
            << "  \"period_frames\": " << options.period_frames << ",\n"
            << "  \"nominal_period_ms\": " << nominal_period_ms << ",\n"
            << "  \"duration_seconds\": " << options.seconds << ",\n"
            << "  \"expected_callbacks\": " << expected_callbacks << ",\n"
            << "  \"callback_count\": " << metrics.callback_count << ",\n"
            << "  \"deadline_misses\": " << metrics.deadline_miss_count << ",\n"
            << "  \"max_callback_gap_ms\": " << metrics.max_callback_gap_ns / 1'000'000.0
            << ",\n"
            << "  \"max_process_time_ms\": " << metrics.max_process_time_ns / 1'000'000.0
            << ",\n"
            << "  \"pass\": " << (passed ? "true" : "false") << "\n"
            << "}\n";
  return passed ? 0 : 2;
}
