'use strict';

var _regenerator = require('babel-runtime/regenerator');

var _regenerator2 = _interopRequireDefault(_regenerator);

var _asyncToGenerator2 = require('babel-runtime/helpers/asyncToGenerator');

var _asyncToGenerator3 = _interopRequireDefault(_asyncToGenerator2);

var _promise = require('babel-runtime/core-js/promise');

var _promise2 = _interopRequireDefault(_promise);

var main = function () {
  var _ref = (0, _asyncToGenerator3.default)( /*#__PURE__*/_regenerator2.default.mark(function _callee() {
    var syncBarrierPayload, transaction, serialized, extension_ratio;
    return _regenerator2.default.wrap(function _callee$(_context) {
      while (1) {
        switch (_context.prev = _context.next) {
          case 0:
            _context.next = 2;
            return sleep(base_period);

          case 2:
            if (!true) {
              _context.next = 17;
              break;
            }

            syncBarrierPayload = {
              seed: exonum.randomUint64()
            };
            transaction = SyncBarrier.create(syncBarrierPayload, exonum.keyPair());
            serialized = transaction.serialize();

            console.log(serialized);

            exonum.send(explorerPath, serialized, 1000, 3000).then(function (obj) {
              return console.log(obj);
            }).catch(function (obj) {
              return console.log(obj);
            });

            _context.next = 10;
            return (0, _utils.get_slack_ratio)().catch(function (err) {
              return console.log(err);
            });

          case 10:
            extension_ratio = _context.sent;


            console.log('extension_ratio = ' + extension_ratio);
            if (extension_ratio > 0.0) {
              current_period = Math.round(base_period * extension_ratio);
            } else {
              current_period = base_period;
            }
            _context.next = 15;
            return sleep(current_period);

          case 15:
            _context.next = 2;
            break;

          case 17:
          case 'end':
            return _context.stop();
        }
      }
    }, _callee, this);
  }));

  return function main() {
    return _ref.apply(this, arguments);
  };
}();

var _exonumClient = require('exonum-client');

var exonum = _interopRequireWildcard(_exonumClient);

var _proto = require('./proto');

var proto = _interopRequireWildcard(_proto);

var _utils = require('./utils');

function _interopRequireWildcard(obj) { if (obj && obj.__esModule) { return obj; } else { var newObj = {}; if (obj != null) { for (var key in obj) { if (Object.prototype.hasOwnProperty.call(obj, key)) newObj[key] = obj[key]; } } newObj.default = obj; return newObj; } }

function _interopRequireDefault(obj) { return obj && obj.__esModule ? obj : { default: obj }; }

require("regenerator-runtime/runtime");

// default period is one minute
var DEFAULT_PERIOD = 60;

//Iteration period in seconds
var base_period = void 0;
if (process.argv.length < 3) {
  base_period = DEFAULT_PERIOD;
} else {
  base_period = parseInt(process.argv[2]);
}

var explorerPath = 'http://127.0.0.1:9000/api/explorer/v1/transactions';

// Numeric identifier of the machinelearning service
var SERVICE_ID = 3;

// Numeric ID of the `TxSyncBarrier` transaction within the service
var SYNCBARRIER_ID = 1;

var SyncBarrier = new exonum.Transaction({
  schema: proto.TxSyncBarrier,
  serviceId: SERVICE_ID,
  methodId: SYNCBARRIER_ID
});

function sleep(time) {
  return new _promise2.default(function (res) {
    return setTimeout(res, time * 1000);
  });
}

var current_period = void 0;


main();