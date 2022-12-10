import sys
import numpy as np
import pandas as pd 
import os
import warnings
# from io import BytesIO
from numproto import ndarray_to_proto, proto_to_ndarray
import tensorflow as tf
import service_pb2 as tx
import time as timer
import pickle

#os.environ["CUDA_VISIBLE_DEVICES"] = "-1"      # To disable using GPU
#tf.get_logger().setLevel('ERROR')
#warnings.filterwarnings('ignore')

MODELS_DIR = '../tx_validator/src/models/'

model_id = sys.argv[6]

data_dir = MODELS_DIR + model_id + '/data.csv'


import importlib
model_mod = importlib.import_module('models.%s.validate'%model_id)

def parse_gradients(gradients_path, isRoundOne):
    if (isRoundOne == "0") :
        print("IN FIRST IF PYTHON")
        gradient = open(gradients_path, "rb").read()
        # print("GRADIENTS: ", gradient)
    
        transaction = tx.TxShareUpdates()
        transaction.ParseFromString(gradient)

        return transaction.gradients
    elif (isRoundOne == "1") :
        # gradient = open(gradients_path, "rb").read()
        # print("BUFFER SIZE: ", len(gradient))
        # array = np.frombuffer(gradient, dtype=np.dtype(np.float32))

        array = np.fromfile(gradients_path, dtype='uint8')

        return array
    



def send_valid(is_valid):
    verdict = 'valid' if is_valid else 'invalid'
    print("VERDICT=" + verdict + "ENDVERDICT")

def send_score(score):
    print("SCORE" + str(score) + "ENDSCORE")

#data_validation = pd.read_csv(data_dir)
gradients = parse_gradients(sys.argv[4], sys.argv[7])
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
    base_model = parse_gradients(sys.argv[3], sys.argv[7])
    evaluate_model_list = list(base_model) + list(gradients)
    evaluate_model = np.array(evaluate_model_list)
# if newModel_flag:
#     np.random.seed(0)
#     base_model = np.random.uniform(low = -0.09, high = 0.09, size = len(gradients)).tolist()
# else: 
#     base_model = parse_gradients(sys.argv[3])
min_score = float(sys.argv[5])
# evaluate_model = base_model + gradients
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
