'use strict';

Object.defineProperty(exports, "__esModule", {
    value: true
});
exports.default = validate_vector;
function validate_vector(newModel_flag, base_model, gradients, validation_path, min_score, onSuccess) {
    run_python(newModel_flag, validation_path, base_model, gradients, min_score).then(function (res) {
        var results = parsePythonVerdict(res.toString());
        onSuccess(results);
    }).catch(function (err) {
        console.error(err);
    });
}

function run_python(newModel_flag, validation_path, base_model, gradients, min_score) {
    var runPy = new Promise(function (success, nosuccess) {
        var _require = require('python-shell'),
            PythonShell = _require.PythonShell;

        var options = {
            mode: 'text',
            scriptPath: '../src/',
            args: [newModel_flag, validation_path, base_model, gradients, min_score, process.argv[5]]
        };
        PythonShell.run('validation_wrapper.py', options, function (err, results) {
            if (err) throw err;
            // results is an array consisting of messages collected during execution
            success(results);
        });
    });
    return runPy;
}

function parsePythonVerdict(pythonOutput) {
    var trimmed = pythonOutput.trim().replace(/(\r\n|\n|\r)/gm, "");
    var st1 = trimmed.search("VERDICT");
    var end1 = trimmed.search("ENDVERDICT");
    var st2 = trimmed.search("SCORE");
    var end2 = trimmed.search("ENDSCORE");
    var verdict = trimmed.substring(st1 + "VERDICT".length, end1);
    var score = trimmed.substring(st2 + "SCORE".length, end2);
    return [verdict, score];
}