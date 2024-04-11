const {list, head, tail} = require('sicp');

// Create the nested list structure
let nestedList = list(1, list(2, list(3, list(4, list(5, list(6, 7))))));

// Extract the value '7' from the nested list
let value = head(tail(head(tail(head(tail(head(tail(head(tail(nestedList))))))))));

console.log(value);
