'use strict';

Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.fetchLatestModel = fetchLatestModel;
exports.fetchMinScore = fetchMinScore;
var http = require('http');

function fetchPortNumber() {
    // console.log(`PROCESS ARGV: ${process.argv[4]}`)
    return 9000 + parseInt(process.argv[4].trim());
}

function HTTPGet(endpointURL) {
    var options = arguments.length > 1 && arguments[1] !== undefined ? arguments[1] : '';

    var getURL = endpointURL + options;
    return new Promise(function (resolve, reject) {
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

var latest_model_index_fmt = function latest_model_index_fmt() {
    var port_number = fetchPortNumber();
    // console.log(`PORT THAT WAS BREAKING THINS: ${port_number}`)
    return 'http://127.0.0.1:' + port_number + '/api/services/ml_service/v1/models/latestmodel';
};
var get_model_by_index_fmt = function get_model_by_index_fmt() {
    var port_number = fetchPortNumber();
    // console.log(`SENDING OUT REQUEST TO GET LATEST MODEL INDEX AT PORT NUMBER ${port_number}`)
    return 'http://127.0.0.1:' + port_number + '/api/services/ml_service/v1/models/getmodel';
};

function getLatestModelIndex() {
    return new Promise(function (resolve, reject) {
        HTTPGet(latest_model_index_fmt()).then(function (res) {
            return resolve(parseInt(res));
        }).catch(function (err) {
            return reject(err);
        });
    });
}

function getModelByIndex(index) {
    var option = '?version=' + index;
    return new Promise(function (resolve, reject) {
        HTTPGet(get_model_by_index_fmt(), option).then(function (res) {
            return resolve(JSON.parse(res).weights);
        }).catch(function (err) {
            return reject(err);
        });
    });
}

function getMinScoreByIndex(index) {
    var option = '?version=' + index;
    return new Promise(function (resolve, reject) {
        HTTPGet(get_model_by_index_fmt(), option).then(function (res) {
            return resolve(JSON.parse(res).min_score);
        }).catch(function (err) {
            return reject(err);
        });
    });
}

function fetchLatestModel() {
    return new Promise(function (resolve, reject) {
        getLatestModelIndex() //retrieve the index of the latest model from the BC
        .then(function (latestIndex) {
            if ([0, -1].includes(latestIndex)) {
                //new model 
                // console.log(`TRYING TO FETCHLATESTMODEL WITH INDEX CORRECT !${latestIndex}`)
                // let zerosArr = new Array(WEIGHTS_LENGTH).fill(0);
                // resolve(zerosArr);

                resolve(0);
            } else {
                // console.log(`TRYING TO FETCHLATESTMODEL WITH INDEX ${latestIndex}`)
                getModelByIndex(latestIndex) //fetch latest model weights
                .then(function (latestModelWeights) {
                    resolve(latestModelWeights);
                }).catch(function (err) {
                    return reject(err);
                });
            }
        }).catch(function (err) {
            return reject(err);
        });
    });
}

function fetchMinScore() {
    return new Promise(function (resolve, reject) {
        getLatestModelIndex() //retrieve the index of the latest model from the BC
        .then(function (latestIndex) {
            if ([0, -1].includes(latestIndex)) {
                //new model 
                var min_score = 0;
                resolve(min_score);
            } else {
                getMinScoreByIndex(latestIndex) //fetch latest model weights
                .then(function (latestMinScore) {
                    resolve(latestMinScore);
                }).catch(function (err) {
                    return reject(err);
                });
            }
        }).catch(function (err) {
            return reject(err);
        });
    });
}