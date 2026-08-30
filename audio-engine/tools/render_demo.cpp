#include "lartycc/audio_engine.hpp"

#include <algorithm>
#include <array>
#include <cmath>
#include <cstdint>
#include <fstream>
#include <numbers>
#include <span>
#include <vector>

namespace {

void write_u16(std::ostream& stream, const std::uint16_t value) {
  const std::array bytes{static_cast<char>(value & 0xffU), static_cast<char>(value >> 8U)};
  stream.write(bytes.data(), bytes.size());
}

void write_u32(std::ostream& stream, const std::uint32_t value) {
  const std::array bytes{static_cast<char>(value & 0xffU),
                         static_cast<char>((value >> 8U) & 0xffU),
                         static_cast<char>((value >> 16U) & 0xffU),
                         static_cast<char>((value >> 24U) & 0xffU)};
  stream.write(bytes.data(), bytes.size());
}

bool write_wav(const char* path, const std::span<const float> stereo) {
  std::ofstream stream(path, std::ios::binary);
  if (!stream) return false;
  const auto data_size = static_cast<std::uint32_t>(stereo.size() * sizeof(std::int16_t));
  stream.write("RIFF", 4);
  write_u32(stream, 36U + data_size);
  stream.write("WAVEfmt ", 8);
  write_u32(stream, 16U);
  write_u16(stream, 1U);
  write_u16(stream, 2U);
  write_u32(stream, 48'000U);
  write_u32(stream, 48'000U * 2U * 2U);
  write_u16(stream, 4U);
  write_u16(stream, 16U);
  stream.write("data", 4);
  write_u32(stream, data_size);
  for (const float value : stereo) {
    const auto sample = static_cast<std::int16_t>(std::clamp(value, -1.0F, 1.0F) * 32'767.0F);
    write_u16(stream, static_cast<std::uint16_t>(sample));
  }
  return stream.good();
}

}  // namespace

int main() {
  constexpr std::size_t sample_rate = 48'000;
  std::vector<float> sample(sample_rate);
  for (std::size_t frame = 0; frame < sample.size(); ++frame) {
    const auto time = static_cast<double>(frame) / sample_rate;
    sample[frame] = static_cast<float>(std::sin(2.0 * std::numbers::pi * 220.0 * time) * 0.25);
  }

  lartycc::audio::AudioEngine engine;
  if (!engine.prepare(sample_rate, 128) || !engine.load_mono_sample(sample) || !engine.play()) {
    return 1;
  }
  std::vector<float> output(sample.size() * 2U);
  for (std::size_t offset = 0; offset < sample.size(); offset += 128) {
    const auto frames = std::min<std::size_t>(128, sample.size() - offset);
    engine.process({sample_rate, frames, 2}, std::span{output}.subspan(offset * 2U, frames * 2U));
  }
  return write_wav("lartycc-demo.wav", output) ? 0 : 1;
}
