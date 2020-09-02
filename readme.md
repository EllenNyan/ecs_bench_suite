# ECS Bench Suite

A suite of benchmarks designed to test and compare Rust ECS library performance across a variety of challenging circumstances.

The full benchmark report is available [here](https://rust-gamedev.github.io/ecs_bench_suite/target/criterion/report/index.html).

|                  | legion (\*)           | legion 0.2.4 | bevy       | hecs    | shipyard (\*)         | specs       |
|------------------|:---------------------:|:------------:|:----------:|:-------:|:---------------------:|:-----------:|
| simple_insert    | **239us**             | 0.968ms      | 1.0865ms   | 496us   | 1.7448ms              | 1.6462ms    |
| simple_iter      | **12.206us** (12.32us)| **12.485us** | **14.27us**| 22.47us | **81.6us** (27.79us)  | 36.938us    |
| frag_iter        | 418.5ns               | 2.28us       | 1.22us     | 1.59us  | **360ns**             | 1.85us      |
| heavy_compute    | **0.596ms** (0.623ms) | 3.648ms      | 1.09ms     | 1.015ms | 0.632ms (0.603ms)     | 0.986ms     |
| schedule         | **50.95us** (52.68us) | 190.7us      | 77.94us    | -       | 611us (224us)         | 228us       |
| add_remove       | 3.20ms                | 2.54ms       | 4.77ms     | 3.57ms  | 268us                 | **112us**   |
| serialize_text   | **12.248ms**          | -            | -          | -       | -                     | -           |
| serialize_binary | **2.696ms**           | -            | -          | -       | -                     | -           |

(*): The values in parentheses are results where per-benchmark storage optimizations were applied. Some of these are mutually exclusive, so with and without "packing" typically represent best and worst-case performance for the ECS.

The best result for each benchmark is marked in bold text. Note that run to run variance for these benchmarks is typically 2-3%, with outliers as much as 10%. All micro-benchmarks should be taken with a grain of salt, and any benchmarks within a few percent of each other should be considered "effectively equal".

## The Benchmarks

### Simple Insert

This benchmark is designed to test the base cost of constructing entities and moving components into the ECS.

Inserts 10,000 entities, each with 4 components: `Transform(mat4x4)`, `Position(vec3)`, `Rotation(vec3)` and `Velocity(vec3)`.

### Simple Iter

This benchmark is designed to test the core overheads involved in component iteration in best-case conditions. The iteration should occur on a single CPU core.

Dataset: 10,000 entities, each with 4 components: `Transform(mat4x4)`, `Position(vec3)`, `Rotation(vec3)` and `Velocity(vec3)`.

Test: Iterate through all entities with `Position` and `Velocity`, and add velocity onto position.

### Fragmented Iter

This benchmark is designed to test how the ECS handles iteration through a fragmented dataset. The iteration should occur on a single CPU core.

Dataset: 26 component types (`A(f32)` through `Z(f32)`), each with 20 entities plus a `Data(f32)` component.

Test: Iterate through all entities with a `Data` component and double its value.

### System Scheduling

This benchmark is designed to test how efficiently the ECS can schedule multiple independent systems on a multi-core CPU. This is primarily an outer-parallelism test. Each system should execute on a single CPU core.

Dataset:

* 10,000 entities with `(A, B)` components.
* 10,000 entities with `(A, B, C)` components.
* 10,000 entities with `(A, B, C, D)` components.
* 10,000 entities with `(A, B, C, E)` components.

Test:

Three systems accessing the following components mutably, where each system swaps the values stored in each component:

* `(A, B)`
* `(C, D)`
* `(C, E)`

### Heavy Compute

This benchmark is designed to test the ECS's ability to scale when it is allowed to run a system over multiple CPU cores. This is primarily an inner-parallelism test.

Dataset: 10,000 entities with a `mat4x4` component.

Test: Iterate through all `mat4x4` components, and invert the matrix 10 times.

### Add/Remove Component

This benchmark is designed to test how quickly the ECS can add and then remove a component from an existing entity.

Dataset: 1,000 entities with a single `A` component.

Test: Iterate through all entities, adding a `B` component. Then iterate through all entities again, removing their `B` component.

### Serialize

This benchmark is designed to test how quickly the ECS and serialize and deserialize its entities in both text (RON) and binary (bincode) formats.

Dataset: 1000 entities with `Transform(mat4x4)`, `Position(vec3)`, `Rotation(vec3)` and `Velocity(vec3)` components.

Test: Serialize all entities to RON and bincode formats in-memory. Then deserialize back into the ECS. The RON and bincode formats should be separate benchmark tests.
