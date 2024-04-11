const { list, head, tail } = require('sicp');

// Creation of nested list
let nestedList = list(1, list(2, list(3, list(4, list(5, list(6, 7))))));

// Extraction of said value 7 from nested list
let value = head(tail(head(tail(head(tail(head(tail(head(tail(nestedList))))))))));

console.log(value);
