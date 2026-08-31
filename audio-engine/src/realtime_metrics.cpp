#include "lartycc/realtime_metrics.hpp"

#include <algorithm>

namespace lartycc::audio {

void RealtimeMetrics::reset() noexcept {
  previous_callback_start_ns_.store(0, std::memory_order_relaxed);
  callback_count_.store(0, std::memory_order_relaxed);
  deadline_miss_count_.store(0, std::memory_order_relaxed);
  max_callback_gap_ns_.store(0, std::memory_order_relaxed);
  max_process_time_ns_.store(0, std::memory_order_relaxed);
}

void RealtimeMetrics::record(const std::uint64_t callback_start_ns,
                             const std::uint64_t callback_finish_ns,
                             const std::uint32_t frames,
                             const std::uint32_t sample_rate) noexcept {
  callback_count_.fetch_add(1, std::memory_order_relaxed);
  const auto previous = previous_callback_start_ns_.exchange(callback_start_ns,
                                                              std::memory_order_relaxed);
  if (previous != 0 && callback_start_ns >= previous) {
    const auto gap = callback_start_ns - previous;
    update_max(max_callback_gap_ns_, gap);
    if (sample_rate != 0) {
      constexpr std::uint64_t nanoseconds_per_second = 1'000'000'000;
      const auto period = static_cast<std::uint64_t>(frames) * nanoseconds_per_second /
                          static_cast<std::uint64_t>(sample_rate);
      if (gap > period + period / 2U) {
        deadline_miss_count_.fetch_add(1, std::memory_order_relaxed);
      }
    }
  }
  if (callback_finish_ns >= callback_start_ns) {
    update_max(max_process_time_ns_, callback_finish_ns - callback_start_ns);
  }
}

RealtimeMetricsSnapshot RealtimeMetrics::snapshot() const noexcept {
  return {
      callback_count_.load(std::memory_order_relaxed),
      deadline_miss_count_.load(std::memory_order_relaxed),
      max_callback_gap_ns_.load(std::memory_order_relaxed),
      max_process_time_ns_.load(std::memory_order_relaxed),
  };
}

void RealtimeMetrics::update_max(std::atomic<std::uint64_t>& target,
                                 const std::uint64_t value) noexcept {
  auto current = target.load(std::memory_order_relaxed);
  while (current < value &&
         !target.compare_exchange_weak(current, value, std::memory_order_relaxed)) {
  }
}

}  // namespace lartycc::audio
