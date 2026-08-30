#include "lartycc/audio_engine.hpp"

#include <array>
#include <cassert>
#include <chrono>

int main() {
  lartycc::audio::AudioEngine engine;
  assert(engine.prepare(48'000.0, 128));
  std::array<float, 48'000> sample{};
  sample.fill(0.25F);
  assert(engine.load_mono_sample(sample));
  assert(engine.play());

  std::array<float, 256> output{};
  const auto started = std::chrono::steady_clock::now();
  for (int block = 0; block < 375; ++block) {
    engine.process({48'000.0, 128, 2}, output);
  }
  const auto elapsed = std::chrono::steady_clock::now() - started;
  assert(engine.position() == sample.size());
  assert(elapsed < std::chrono::seconds(1));
}
