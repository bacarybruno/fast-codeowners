# fast-codeowners

A library for GitHub's [CODEOWNERS file](https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/about-code-owners#codeowners-syntax).

Based on [https://github.com/hmarr/codeowners-rs](https://github.com/hmarr/codeowners-rs), written in 🦀 by [hmarr](https://github.com/hmarr), and exposed as a JS library using [NAPI-RS](https://napi.rs/).

![https://github.com/bacarybruno/fast-codeowners/actions](https://github.com/bacarybruno/fast-codeowners/workflows/CI/badge.svg)

### Benchmark
Compared to the popular [codeowners npm package](https://www.npmjs.com/package/codeowners) npm package `fast-codeowners` performs `13.5x` faster in throughput benchmarks.
- fast-codeowners: 37,011 ops/sec
- codeowners (npm): 2,740 ops/sec

```
┌─────────┬───────────────────┬──────────────────┬──────────────────┬────────────────────────┬────────────────────────┬─────────┐
│ (index) │ Task name         │ Latency avg (ns) │ Latency med (ns) │ Throughput avg (ops/s) │ Throughput med (ops/s) │ Samples │
├─────────┼───────────────────┼──────────────────┼──────────────────┼────────────────────────┼────────────────────────┼─────────┤
│ 0       │ 'fast-codeowners' │ '27791 ± 0.30%'  │ '26666 ± 1749.0' │ '37011 ± 0.13%'        │ '37501 ± 2499'         │ 35983   │
│ 1       │ 'codeowners (npm' │ '367847 ± 0.38%' │ '358917 ± 11292' │ '2740 ± 0.30%'         │ '2786 ± 88'            │ 2719    │
└─────────┴───────────────────┴──────────────────┴──────────────────┴────────────────────────┴────────────────────────┴─────────┘
```