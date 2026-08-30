# Real-time safety checklist

Before merging callback-path code, prove that buffers and graph state are
prepared off-thread; execution is bounded; no allocation, deallocation, I/O,
logging, exception, lock, or reference-count destruction is possible; queue
overflow behavior is tested; invalid/denormal floats are handled; and callback
duration has a regression benchmark. See `ARCHITECTURE.md` for the threading
contract.

