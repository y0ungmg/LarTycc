#pragma once

#include <atomic>
#include <cstdint>

namespace lartycc::audio {

struct RealtimeMetricsSnapshot {
  std::uint64_t callback_count{0};
  std::uint64_t deadline_miss_count{0};
  std::uint64_t max_callback_gap_ns{0};
  std::uint64_t max_process_time_ns{0};
};

// Lock-free counters for opt-in callback timing qualification.
class RealtimeMetrics final {
 public:
  void reset() noexcept;
  void record(std::uint64_t callback_start_ns, std::uint64_t callback_finish_ns,
              std::uint32_t frames, std::uint32_t sample_rate) noexcept;
  [[nodiscard]] RealtimeMetricsSnapshot snapshot() const noexcept;

 private:
  static void update_max(std::atomic<std::uint64_t>& target,
                         std::uint64_t value) noexcept;

  std::atomic<std::uint64_t> previous_callback_start_ns_{0};
  std::atomic<std::uint64_t> callback_count_{0};
  std::atomic<std::uint64_t> deadline_miss_count_{0};
  std::atomic<std::uint64_t> max_callback_gap_ns_{0};
  std::atomic<std::uint64_t> max_process_time_ns_{0};
};

}  // namespace lartycc::audio
