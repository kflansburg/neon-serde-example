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
};

const output = getSeedData();
// console.log();
// console.log(addon.hello(object));

const iterJs = (input) => {
  let result = [];
  const output = JSON.parse(JSON.stringify(input));
  Object.entries(output).map(([pagekey, page]) => {
    Object.entries(page).map(([groupKey, group]) => {
      // console.log(`${groupKey} is "${pagekey}"`);
      group.map((stuff) => {
        result.push({ pagekey, groupKey, stuff });
      });
    });
  });
  return result;
};

const Benchmark = require("benchmark");
var suite = new Benchmark.Suite();
suite
  .add("javascript", () => {
    iterJs(object);
  })
  .add("native.module", () => {
    addon.hello(object);
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
