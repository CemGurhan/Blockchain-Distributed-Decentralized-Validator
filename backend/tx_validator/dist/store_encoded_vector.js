'use strict';

Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.default = store_encoded_vector;
exports.clear_encoded_vector = clear_encoded_vector;
var fs = require('fs');

function store_encoded_vector(gradients, filename) {
    return new Promise(function (resolve) {
        // let encoded = gradients.join("|"); 
        fs.writeFileSync(filename, gradients);
        resolve(filename);
    });
}

function clear_encoded_vector(filename) {
    fs.unlink(filename, function () {});
}