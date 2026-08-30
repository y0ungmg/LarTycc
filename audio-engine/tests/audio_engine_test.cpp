#include "lartycc/audio_engine.hpp"

#include <array>
#include <cassert>

int main() {
  lartycc::audio::AudioEngine engine;
  assert(!engine.is_prepared());
  assert(!engine.prepare(0.0, 512));
  assert(engine.prepare(48'000.0, 512));

  constexpr std::array<float, 4> sample{0.25F, -0.5F, 1.0F, 0.0F};
  assert(engine.load_mono_sample(sample));
  assert(engine.sample_length() == 4);
  engine.set_master_gain(0.5F);
  assert(engine.play());

  std::array<float, 8> output{1.0F, 1.0F, 1.0F, 1.0F,
                              1.0F, 1.0F, 1.0F, 1.0F};
  engine.process({48'000.0, 4, 2}, output);
  constexpr std::array<float, 8> expected{0.125F, 0.125F, -0.25F, -0.25F,
                                          0.5F, 0.5F, 0.0F, 0.0F};
  assert(output == expected);
  assert(engine.position() == 4);
  assert(!engine.is_playing());
  assert(engine.seek(1));
  assert(!engine.seek(5));
}
