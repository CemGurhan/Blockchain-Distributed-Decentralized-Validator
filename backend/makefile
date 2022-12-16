four:
	sh test_scripts/run_nondis_test.sh -n 4 -t 1   
.PHONY:four

four-SSP:
	sh test_scripts/run_nondis_test.sh -n 4 -t 1 -s SSP
.PHONY:four-SSP

four-BSP:
	sh test_scripts/run_nondis_test.sh -n 4 -t 1 -s BSP
.PHONY:four-BSP

n-four:
	sh test_scripts/run_nondis_test.sh -n 4   
.PHONY:n-four

n-four-SSP:
	sh test_scripts/run_nondis_test.sh -n 4 -s SSP
.PHONY:n-four-SSP

n-four-BSP:
	sh test_scripts/run_nondis_test.sh -n 4 -s BSP
.PHONY:n-four-BSP

proto:
	protoc -I=./src/proto  --python_out=./tx_validator/src ./src/proto/service.proto
.PHONY:proto





	