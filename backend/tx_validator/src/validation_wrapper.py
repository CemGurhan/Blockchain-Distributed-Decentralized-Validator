import sys
import numpy as np
import service_pb2 as tx
import requests

MODELS_DIR = '../tx_validator/src/models/'

newModelFlag = str(sys.argv[1])
gradientsArrayFilePath = sys.argv[2]
minScoreString = sys.argv[3]
modelName = sys.argv[4]
modelIndex = sys.argv[5]

data_dir = MODELS_DIR + modelName + '/data.csv'


import importlib
model_mod = importlib.import_module('models.%s.validate'%modelName)

def parse_gradients_round_one(gradientsArrayFilePath):
    gradient = open(gradientsArrayFilePath, "rb").read()
    transaction = tx.TxShareUpdates()
    transaction.ParseFromString(gradient)
     
    return transaction.gradients
    

def parse_gradients_round_two_onwards():
    formattedURL = "http://127.0.0.1:9000/api/services/ml_service/v1/models/latestmodel_raw?version={}".format(modelIndex)
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

gradients = parse_gradients_round_one(gradientsArrayFilePath)
if (newModelFlag == "true"):
    newModelFlag = 1
elif (newModelFlag == "false"):
    newModelFlag = 0
else:
    newModelFlag = int(newModelFlag)

if newModelFlag:
    evaluate_model = gradients
else:
    base_model = parse_gradients_round_two_onwards()
    evaluate_model = base_model + gradients
   
minScoreFloat = float(minScoreString)
score = model_mod.compute_validation_score(evaluate_model, data_dir)
is_valid = score >= minScoreFloat

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
