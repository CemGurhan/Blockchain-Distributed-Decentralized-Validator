four:
	sh test_scripts/run_nondis_test.sh -n 4 -t 1   
.PHONY:four

n-four:
	sh test_scripts/run_nondis_test.sh -n 4   
.PHONY:n-four

proto:
	protoc -I=./src/proto  --python_out=./tx_validator/src ./src/proto/service.proto
.PHONY:proto

push:
	git push https://github.com/PHilitaXDataNET/DataNET-validator-NDC NONDIS:main
.PHONY:push




	