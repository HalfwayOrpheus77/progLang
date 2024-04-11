const s = require('sicp');
const { list, head, tail } = s.pairs;

// Create the nested list
const nestedList = list(1, list(2, list(3, list(4, list(5, list(6, 7))))));

// Extract the value '7'
const value = head(tail(tail(tail(tail(tail(nestedList))))));

console.log(value);

