#include "lartycc/audio_engine.hpp"

#include <array>
#include <cassert>

int main() {
  lartycc::audio::AudioEngine engine;
  assert(!engine.is_prepared());
  assert(!engine.prepare(0.0, 512));
  assert(engine.prepare(48'000.0, 512));

  std::array<float, 8> output{1.0F, 1.0F, 1.0F, 1.0F,
                              1.0F, 1.0F, 1.0F, 1.0F};
  engine.process({48'000.0, 4}, output);
  for (const float sample : output) {
    assert(sample == 0.0F);
  }
}

