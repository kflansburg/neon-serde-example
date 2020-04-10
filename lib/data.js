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

function getSeedData() {
  /**
   * SEED DATA
   */
  const output = {
    page1: {
      group1: [],
    },
  };

  // 1M data
  for (let i = 1; i <= 1000000; i++) {
    output.page1.group1.push({
      Title: "Example Domain",
      Link: "http://example.com/?view=" + i,
    });
  }

  return output;
}

module.exports = getSeedData();