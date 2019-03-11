/*
Perf Benchmark
*/

const DEFAULT_ITERATION_COUNT = 10;

export default (func, input, iterationCount = DEFAULT_ITERATION_COUNT) => {
  const start = performance.now();
  for (let i = 0; i < iterationCount; i++) {
    func(input);
  }
  const finish = performance.now();
  return (finish - start) / DEFAULT_ITERATION_COUNT;
};
