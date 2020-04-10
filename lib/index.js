var addon = require("../native");

function getSeedData() {
  /**
   * SEED DATA
   */
  const output = {
    page1: {
      group1: [],
    },
  };

  for (let i = 1; i <= 1000; i++) {
    output.page1.group1.push({
      Title: "Example Domain",
      Link: "http://example.com/?view=" + i,
    });
  }

  return output;
}

const object = {
  page1: {
    group1: [
      { name: "name1", index: 1 },
      { name: "name33", index: 33 },
      { name: "name45", index: 45 },
    ],
  },
  page500: {
    group25: [
      { name: "name1", index: 1 },
      { name: "name33", index: 33 },
      { name: "name45", index: 45 },
    ],
  },
};

const output = getSeedData();

const iterJs = (inputRef) => {
  let result = [];
  const input = JSON.parse(JSON.stringify(inputRef));
  Object.entries(input).map(([page_key, page]) => {
    Object.entries(page).map(([group_key, group]) => {
      group.map((stuff) => {
        result.push({ page_key, group_key, stuff });
      });
    });
  });
  return result;
};

const iterRust = (input) => {
  return JSON.parse(addon.hello(JSON.stringify(input)));
};

console.time("iterJS");
iterJs(object);
console.timeEnd("iterJS");

console.time("iterRust");
iterRust(object);
console.timeEnd("iterRust");

const Benchmark = require("benchmark");
var suite = new Benchmark.Suite();
suite
  .add("native.module", () => {
    iterRust(object);
  })
  .add("javascript", () => {
    iterJs(object);
  })
  // add listeners
  .on("cycle", function (event) {
    console.log(String(event.target));
  })
  .on("complete", function () {
    console.log("Fastest is " + this.filter("fastest").map("name"));
  })
  // run async
  .run({ async: true });
