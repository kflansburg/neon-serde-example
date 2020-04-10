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

const iterJs = (output) => {
  let result = [];
  // const output = JSON.parse(JSON.stringify(input));
  Object.entries(output).map(([pagekey, page]) => {
    Object.entries(page).map(([groupKey, group]) => {
      group.map((stuff) => {
        result.push({ pagekey, groupKey, stuff });
      });
    });
  });
  return result;
};

console.time('iterJS');
// console.log(iterJs(object));
iterJs(object);
console.timeEnd('iterJS');
// // console.log(JSON.parse(JSON.parse(addon.hello(JSON.stringify(object)))));

console.time('iterRust');
JSON.parse(addon.hello(JSON.stringify(object)));
console.timeEnd('iterRust');

console.time('hellojson');
addon.hellojson();
console.timeEnd('hellojson');

console.time('hello2');
// console.log(JSON.stringify(object));
addon.hello2(JSON.stringify(object));
console.timeEnd('hello2');

// const Benchmark = require("benchmark");
// var suite = new Benchmark.Suite();
// suite
//   .add("native.module", () => {
//     JSON.parse(addon.hello(JSON.stringify(object)));
//   })
//   // .add("javascript", () => {
//   //   iterJs(object);
//   // })
//   // // add listeners
//   .on("cycle", function (event) {
//     console.log(String(event.target));
//   })
//   .on("complete", function () {
//     console.log("Fastest is " + this.filter("fastest").map("name"));
//   })
//   // run async
//   .run({ async: true });
