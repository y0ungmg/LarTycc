#include "lartycc/audio_host_c.h"

#include <array>
#include <cassert>

int main() {
  auto* host = lartycc_audio_host_create();
  assert(host != nullptr);
  (void)lartycc_audio_host_available(host);

  const auto count = lartycc_audio_host_refresh_devices(host);
  if (count > 0) {
    std::array<char, 1024> id{};
    std::array<char, 1024> name{};
    bool is_default = false;
    assert(lartycc_audio_host_device_info(host, 0, id.data(), id.size(), name.data(),
                                         name.size(), &is_default));
    assert(name[0] != '\0');
  }

  constexpr std::array<float, 4> sample{0.0F, 0.5F, -0.5F, 0.0F};
  assert(lartycc_audio_host_load_mono(host, sample.data(), sample.size()));
  assert(!lartycc_audio_host_play(host));
  assert(lartycc_audio_host_seek(host, 2));
  assert(lartycc_audio_host_position(host) == 2);
  lartycc_audio_host_set_gain(host, 0.5F);
  lartycc_audio_host_stop(host);
  assert(!lartycc_audio_host_is_playing(host));

  lartycc_audio_host_destroy(host);
}
