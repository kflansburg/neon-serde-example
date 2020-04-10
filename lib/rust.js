const data = require('./data');

var addon = require("../native");

JSON.parse(addon.hello(JSON.stringify(data)))