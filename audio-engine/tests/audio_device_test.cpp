#include "lartycc/audio_device.hpp"
#include "lartycc/audio_engine.hpp"

#include <cassert>

int main() {
  lartycc::audio::AudioEngine engine;
  lartycc::audio::AudioOutput output(engine);
  assert(output.available());
  assert(!output.is_running());
  const auto devices = output.devices();
  for (const auto& device : devices) {
    assert(!device.id.empty());
    assert(!device.name.empty());
  }
}
