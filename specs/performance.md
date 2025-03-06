# Performance Specification

## Overview

This document outlines performance requirements, optimization strategies, and benchmarking approaches for our roguelike game.

## Performance Targets

### 1. Frame Time Budget

```rust
pub const FRAME_TIME_TARGET: Duration = Duration::from_millis(16); // 60 FPS
pub const FRAME_TIME_WARNING: Duration = Duration::from_millis(20);
pub const FRAME_TIME_CRITICAL: Duration = Duration::from_millis(33);
```

### 2. System Budgets

- Input Processing: 1ms
- Game Logic: 4ms
- Pathfinding: 3ms
- FOV Calculation: 2ms
- Rendering: 6ms

### 3. Memory Targets

- Base Memory: < 50MB
- Peak Memory: < 200MB
- Texture Memory: < 32MB
- Stack Usage: < 8MB

## Optimization Strategies

### 1. Data Structures

```rust
// Spatial Hashing for Entity Lookup
pub struct SpatialHash {
    cells: HashMap<(i32, i32), Vec<Entity>>,
    cell_size: i32,
}

// Component Storage Optimization
pub struct DenseStorage<T> {
    data: Vec<T>,
    entity_map: HashMap<Entity, usize>,
}
```

### 2. Algorithm Optimization

```rust
// Cached Pathfinding
pub struct PathCache {
    paths: LruCache<(Point, Point), Vec<Point>>,
    dijkstra_maps: LruCache<Point, DijkstraMap>,
}

// Incremental FOV Updates
pub struct FovSystem {
    dirty_positions: HashSet<Position>,
    fov_cache: HashMap<Entity, FieldOfView>,
}
```

## Profiling Infrastructure

### 1. Performance Metrics

```rust
pub struct PerformanceMetrics {
    frame_times: RingBuffer<Duration>,
    system_times: HashMap<String, RingBuffer<Duration>>,
    memory_usage: MemoryMetrics,
}

pub struct MemoryMetrics {
    heap_used: usize,
    stack_used: usize,
    texture_memory: usize,
}
```

### 2. Profiling Macros

```rust
macro_rules! profile_scope {
    ($name:expr) => {
        let _guard = ProfileGuard::new($name);
    };
}

macro_rules! profile_fn {
    () => {
        profile_scope!(function_name!())
    };
}
```

## Memory Management

### 1. Resource Pooling

```rust
pub struct ObjectPool<T> {
    available: Vec<T>,
    in_use: HashMap<Entity, T>,
    create_fn: Box<dyn Fn() -> T>,
}
```

### 2. Memory Budgets

```rust
pub struct MemoryBudget {
    component_allocations: HashMap<TypeId, usize>,
    system_allocations: HashMap<String, usize>,
    texture_budget: usize,
}
```

## System Optimization

### 1. Query Optimization

```rust
// Archetype-based Query
pub struct OptimizedQuery<T: Component> {
    archetypes: Vec<ArchetypeAccess>,
    _marker: PhantomData<T>,
}

// Cached Query Results
pub struct QueryCache<T> {
    results: Vec<QueryResult<T>>,
    generation: u32,
}
```

### 2. Parallel Processing

```rust
pub struct ParallelSystem {
    chunk_size: usize,
    thread_count: usize,
    work_queue: WorkQueue,
}
```

## Testing and Benchmarking

### 1. Benchmark Suite

```rust
pub struct BenchmarkSuite {
    scenarios: Vec<BenchmarkScenario>,
    metrics: Vec<MetricCollector>,
    results: BenchmarkResults,
}
```

### 2. Performance Tests

```rust
#[cfg(test)]
mod performance_tests {
    #[test]
    fn test_pathfinding_performance() {
        // Test pathfinding with various map sizes
    }

    #[test]
    fn test_fov_calculation_performance() {
        // Test FOV calculation with different numbers of entities
    }
}
```

## Monitoring and Alerts

### 1. Performance Monitoring

```rust
pub struct PerformanceMonitor {
    metrics: PerformanceMetrics,
    alerts: Vec<Alert>,
    thresholds: PerformanceThresholds,
}
```

### 2. Alert System

```rust
pub enum AlertType {
    FrameTimeExceeded,
    MemoryThresholdReached,
    SystemSlowdown,
    ResourceExhaustion,
}
```

## Development Guidelines

### 1. Code Optimization

- Profile before optimizing
- Use appropriate data structures
- Minimize allocations
- Batch operations

### 2. Memory Management

- Pool frequently allocated objects
- Use arena allocators
- Implement proper cleanup
- Monitor memory usage

### 3. System Design

- Efficient component access
- Parallel processing where possible
- Smart caching strategies
- Minimal cross-system dependencies

### 4. Documentation

- Performance implications
- Memory requirements
- Optimization notes
- Benchmark results

## Performance Monitoring Tools

### 1. In-Game Profiler

```rust
pub struct InGameProfiler {
    enabled: bool,
    metrics: PerformanceMetrics,
    display: ProfilerDisplay,
}
```

### 2. Logging System

```rust
pub struct PerformanceLogger {
    log_level: LogLevel,
    metrics: Vec<MetricType>,
    output: Box<dyn Write>,
}
```
