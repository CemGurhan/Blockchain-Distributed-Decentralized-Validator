four:
	sh test_scripts/local_testing/run_local_test.sh 4  
.PHONY:four

proto:
	protoc -I=./src/proto  --python_out=./tx_validator/src ./src/proto/service.proto
.PHONY:proto
	