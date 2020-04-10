const data = require('./data');

var addon = require("../native");

const iterRust = (input) => {
  return addon.hello(JSON.stringify(input));
};

iterRust(data);