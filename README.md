# fast-codeowners

A library for GitHub's [CODEOWNERS file](https://docs.github.com/en/github/creating-cloning-and-archiving-repositories/about-code-owners#codeowners-syntax).

Based on [https://github.com/hmarr/codeowners-rs](https://github.com/hmarr/codeowners-rs), written in 🦀 by [hmarr](https://github.com/hmarr), and exposed as a JS library using [NAPI-RS](https://napi.rs/).

![https://github.com/bacarybruno/fast-codeowners/actions](https://github.com/bacarybruno/fast-codeowners/workflows/CI/badge.svg)

### Benchmark
Compared to the popular [codeowners npm package](https://www.npmjs.com/package/codeowners) npm package `fast-codeowners` performs around `14x` faster in throughput benchmarks.
- fast-codeowners: 38k ops/sec
- codeowners (npm): 2,7k ops/sec

```
┌─────────┬────────────────────┬──────────────────┬──────────────────┬────────────────────────┬────────────────────────┬─────────┐
│ (index) │ Task name          │ Latency avg (ns) │ Latency med (ns) │ Throughput avg (ops/s) │ Throughput med (ops/s) │ Samples │
├─────────┼────────────────────┼──────────────────┼──────────────────┼────────────────────────┼────────────────────────┼─────────┤
│ 0       │ 'fast-codeowners'  │ '27102 ± 0.28%'  │ '26250 ± 1583.0' │ '37716 ± 0.12%'        │ '38095 ± 2309'         │ 36898   │
│ 1       │ 'codeowners (npm)' │ '376665 ± 0.53%' │ '361250 ± 12542' │ '2692 ± 0.39%'         │ '2768 ± 97'            │ 2655    │
└─────────┴────────────────────┴──────────────────┴──────────────────┴────────────────────────┴────────────────────────┴─────────┘
```