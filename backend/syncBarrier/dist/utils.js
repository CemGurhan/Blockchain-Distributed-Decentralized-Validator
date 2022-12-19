"use strict";

Object.defineProperty(exports, "__esModule", {
    value: true
});

var _promise = require("babel-runtime/core-js/promise");

var _promise2 = _interopRequireDefault(_promise);

exports.get_slack_ratio = get_slack_ratio;

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

var http = require('http');

var EXPLORER_ADDR = "http://127.0.0.1:9000";
var SERVICE_DIR = "api/services/ml_service/v1";

var GET_SLACK_RATIO_URL = EXPLORER_ADDR + "/" + SERVICE_DIR + "/sync/slack_ratio";

function HTTPGet(endpointURL) {
    var options = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : '';

    var getURL = endpointURL + options;
    return new _promise2.default(function (resolve, reject) {
        var request = http.get(getURL, function (resp) {
            var data = '';
            resp.on('data', function (chunk) {
                data += chunk;
            });

            resp.on('end', function () {
                resolve(data);
            });
        });

        request.on("error", function (err) {
            reject("Error: " + err.message);
        });
    });
}

// Wire API returning the remaining participation
function get_slack_ratio() {
    return new _promise2.default(function (resolve, reject) {
        HTTPGet(GET_SLACK_RATIO_URL).then(function (res) {
            return resolve(parseFloat(res));
        }).catch(function (err) {
            return reject(err);
        });
    });
}