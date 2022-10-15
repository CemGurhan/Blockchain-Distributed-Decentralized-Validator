'use strict';

Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.default = evaluate_model;
function evaluate_model(base_model, onSuccess) {
    run_python(base_model).then(function (res) {
        var score = parsePythonOutput(res.toString());
        onSuccess(score);
    }).catch(function (err) {
        console.error(err);
    });
}

function run_python(base_model) {
    var runPy = new Promise(function (success, nosuccess) {
        var _require = require('python-shell'),
            PythonShell = _require.PythonShell;

        var options = {
            mode: 'text',
            scriptPath: '../src/',
            args: [base_model]
        };
        PythonShell.run('evaluation_wrapper.py', options, function (err, results) {
            if (err) throw err;
            // results is an array consisting of messages collected during execution
            success(results);
        });
    });
    return runPy;
}

function parsePythonOutput(pythonOutput) {
    var trimmed = pythonOutput.trim().replace(/(\r\n|\n|\r)/gm, "");
    var st = trimmed.search("RETURN");
    var end = trimmed.search("ENDRETURN");
    return trimmed.substring(st + 6, end);
}