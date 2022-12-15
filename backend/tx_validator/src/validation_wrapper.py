import sys
import numpy as np
import service_pb2 as tx
import requests

MODELS_DIR = '../tx_validator/src/models/'

model_id = sys.argv[6]

data_dir = MODELS_DIR + model_id + '/data.csv'


import importlib
model_mod = importlib.import_module('models.%s.validate'%model_id)

def parse_gradients(gradients_path, isRoundOne):
    if (isRoundOne == "1") : 
        gradient = open(gradients_path, "rb").read()
        transaction = tx.TxShareUpdates()
        transaction.ParseFromString(gradient)
     
        return transaction.gradients
    elif (isRoundOne == "0") :
        formattedURL = "http://127.0.0.1:9000/api/services/ml_service/v1/models/latestmodel_raw?version={}".format(sys.argv[8])
        response = requests.get(formattedURL)

        latest_model = response.text
        latest_model_no_prefix = latest_model.replace("[","")
        latest_model_no_suffix = latest_model_no_prefix.replace("]","")

        latest_model_list = latest_model_no_suffix.split(",")
        split = [float(element) for element in latest_model_list]
        return np.array(split)

def send_valid(is_valid):
    verdict = 'valid' if is_valid else 'invalid'
    print("VERDICT=" + verdict + "ENDVERDICT")

def send_score(score):
    print("SCORE" + str(score) + "ENDSCORE")

gradients = parse_gradients(sys.argv[4], "1")
newModel_flag = str(sys.argv[1])
if (newModel_flag == "true"):
    newModel_flag = 1
elif (newModel_flag == "false"):
    newModel_flag = 0
else:
    newModel_flag = int(newModel_flag)

if newModel_flag:
    evaluate_model = gradients
else:
    base_model = parse_gradients(sys.argv[3], "0")
    evaluate_model = base_model + gradients
   
min_score = float(sys.argv[5])
score = model_mod.compute_validation_score(evaluate_model, data_dir)
is_valid = score >= min_score

send_valid(is_valid)
send_score(score)

'''

# TODO: Validation using data_validation and gradients vector
# Call "send_valid" with True if valid and False otherwise

# Placeholder: valid vector only if all are below 1
if max(gradients >= 1.0):
    send_valid(False)
else: 
    send_valid(True)

'''
