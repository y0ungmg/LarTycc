#pragma once

#include <stdbool.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct lartycc_audio_host lartycc_audio_host;

lartycc_audio_host* lartycc_audio_host_create(void);
void lartycc_audio_host_destroy(lartycc_audio_host* host);

bool lartycc_audio_host_available(const lartycc_audio_host* host);
size_t lartycc_audio_host_refresh_devices(lartycc_audio_host* host);
bool lartycc_audio_host_device_info(const lartycc_audio_host* host, size_t index,
                                    char* id, size_t id_capacity,
                                    char* name, size_t name_capacity,
                                    bool* is_default);

bool lartycc_audio_host_load_mono(lartycc_audio_host* host,
                                  const float* samples, size_t sample_count);
bool lartycc_audio_host_start(lartycc_audio_host* host, const char* device_id,
                              unsigned int sample_rate,
                              unsigned int period_frames);
void lartycc_audio_host_stop(lartycc_audio_host* host);
bool lartycc_audio_host_play(lartycc_audio_host* host);
bool lartycc_audio_host_seek(lartycc_audio_host* host, size_t frame);
void lartycc_audio_host_set_gain(lartycc_audio_host* host, float gain);
bool lartycc_audio_host_is_playing(const lartycc_audio_host* host);
size_t lartycc_audio_host_position(const lartycc_audio_host* host);
size_t lartycc_audio_host_callback_count(const lartycc_audio_host* host);

#ifdef __cplusplus
}
#endif
