// Measures method dispatch cost across class inheritance hierarchies
// of varying depth: own method, 1-hop, 2-hop, and 3-hop prototype chains.
// Related to #4265 (transitive prototype inline caching).
const kIterations = 100_000;

function Base() {}
Base.prototype.method = function () {
  return 1;
};

function Depth1() {}
Depth1.prototype = Object.create(Base.prototype);
Depth1.prototype.constructor = Depth1;

function Depth2() {}
Depth2.prototype = Object.create(Depth1.prototype);
Depth2.prototype.constructor = Depth2;

function Depth3() {}
Depth3.prototype = Object.create(Depth2.prototype);
Depth3.prototype.constructor = Depth3;

function Own() {}
Own.prototype.method = function () {
  return 1;
};

let own = new Own();
let d1 = new Depth1();
let d2 = new Depth2();
let d3 = new Depth3();

function main() {
  let sum = 0;

  for (let i = 0; i < kIterations; i++) {
    sum += own.method();
  }

  for (let i = 0; i < kIterations; i++) {
    sum += d1.method();
  }

  for (let i = 0; i < kIterations; i++) {
    sum += d2.method();
  }

  for (let i = 0; i < kIterations; i++) {
    sum += d3.method();
  }

  return sum;
}
