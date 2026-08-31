#include "lartycc/realtime_metrics.hpp"

#include <cassert>

int main() {
  lartycc::audio::RealtimeMetrics metrics;
  metrics.record(1'000'000, 1'100'000, 480, 48'000);
  metrics.record(11'000'000, 11'200'000, 480, 48'000);
  metrics.record(31'000'000, 31'300'000, 480, 48'000);

  const auto snapshot = metrics.snapshot();
  assert(snapshot.callback_count == 3);
  assert(snapshot.deadline_miss_count == 1);
  assert(snapshot.max_callback_gap_ns == 20'000'000);
  assert(snapshot.max_process_time_ns == 300'000);

  metrics.reset();
  const auto reset = metrics.snapshot();
  assert(reset.callback_count == 0);
  assert(reset.deadline_miss_count == 0);
  assert(reset.max_callback_gap_ns == 0);
  assert(reset.max_process_time_ns == 0);
}
