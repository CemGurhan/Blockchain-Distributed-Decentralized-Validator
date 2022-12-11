'use strict';

var _exonumClient = require('exonum-client');

var exonum = _interopRequireWildcard(_exonumClient);

var _proto = require('./proto');

var proto = _interopRequireWildcard(_proto);

var _validate_vector = require('./validate_vector');

var _validate_vector2 = _interopRequireDefault(_validate_vector);

var _store_encoded_vector = require('./store_encoded_vector');

var _store_encoded_vector2 = _interopRequireDefault(_store_encoded_vector);

var _utils = require('./utils');

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

function _interopRequireWildcard(obj) { if (obj && obj.__esModule) { return obj; } else { var newObj = {}; if (obj != null) { for (var key in obj) { if (Object.prototype.hasOwnProperty.call(obj, key)) newObj[key] = obj[key]; } } newObj.default = obj; return newObj; } }

function validation() {
  var transaction = proto.TxShareUpdates.decode(exonum.hexadecimalToUint8Array(process.argv[3]));
  var min_score = transaction.min_score;
  (0, _store_encoded_vector2.default)(transaction.gradients).then(function (encoded_vector_path) {
    (0, _validate_vector2.default)(encoded_vector_path, process.argv[2], 0, function (valid) {
      (0, _store_encoded_vector.clear_encoded_vector)();

      if (valid) {
        console.log("VALID");
      } else {
        console.log("INVALID");
      }
    });
  });
}

(0, _utils.fetchLatestModel)().then(function (base_model) {
  (0, _utils.fetchMinScore)().then(function (min_score) {
    var fs = require("fs");
    // var text = fs.readFileSync("../../example/"+process.argv[3], {encoding:"utf8"});
    // text = text.replace("[", "")
    // text = text.replace("]", "")
    // text = text.split(",")
    // text = text.map(Number);


    var binary_weights = fs.readFileSync("../../example/" + process.argv[3]);
    var transaction = proto.TxShareUpdates.decode(binary_weights);
    // delete a file
    fs.unlink("../../example/" + process.argv[3], function (err) {
      if (err) {
        throw err;
      }
    });
    // let transaction = proto.TxShareUpdates.decode(text);
    // console.log("First element = ",transaction.gradients[0])
    // console.log("Last element = ",transaction.gradients[transaction.gradients.length-1])
    var val_id = process.argv[4];
    (0, _store_encoded_vector2.default)(transaction.gradients, "gradients" + val_id).then(function (encoded_vector_path) {
      if (base_model == 0) {
        (0, _validate_vector2.default)(1, "", encoded_vector_path, process.argv[2], min_score, function (results) {
          (0, _store_encoded_vector.clear_encoded_vector)("gradients" + val_id);
          (0, _store_encoded_vector.clear_encoded_vector)("basemodel" + val_id);

          var verdict = results[0];
          var score = results[1];
          console.log(verdict.toUpperCase() + ':' + score);
        });
      } else {
        (0, _store_encoded_vector2.default)(base_model, "basemodel" + val_id).then(function (encoded_basemodel_path) {
          (0, _validate_vector2.default)(0, encoded_basemodel_path, encoded_vector_path, process.argv[2], min_score, function (results) {
            (0, _store_encoded_vector.clear_encoded_vector)("gradients" + val_id);
            (0, _store_encoded_vector.clear_encoded_vector)("basemodel" + val_id);

            var verdict = results[0];
            var score = results[1];
            console.log(verdict.toUpperCase() + ':' + score);
          });
        });
      }
    });
  });
});