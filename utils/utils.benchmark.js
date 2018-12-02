export default (func, input, iterationCount) => {
  const start = performance.now();
  for (let i = 0; i < iterationCount; i++) {
    func(input);
  }
  const finish = performance.now();
  return finish - start;
};
