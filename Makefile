MAKEFLAGS += --silent

CL_REPO_DIR = "target/CasperLabs"
CL_DOCKER_TAG = "release-v0.7"
CL_GIT_TAG = "release-v0.7"
CL_GIT_REPO = "https://github.com/CasperLabs/CasperLabs.git"
IT = "$(shell pwd)/${CL_REPO_DIR}/integration-testing"
PIPENV_LOCK_FILE = "$(shell pwd)/target/installed-pipenv-requirements.txt"

all: prepare

# Prepare local environment.
prepare: \
	download-casperlabs-repository \
	pull-casperlabs-release-dockers \
	build-test-docker-images \
	build-python-environment \
	build-test-wasm-contracts

# Download CasperLabs reposotory and place it target directory.
download-casperlabs-repository:
	if [ ! -d ${CL_REPO_DIR} ]; then                                      \
		echo "[x] Downloading CasperLabs repository..."	                ; \
		mkdir -p ${CL_REPO_DIR}                                         ; \
		git clone -b ${CL_GIT_TAG} ${CL_GIT_REPO} $(CL_REPO_DIR)        ; \
	fi
	echo "[x] CasperLabs repository with tag: ${CL_GIT_TAG}... present."

# Download Docker images with a given release tag.
pull-casperlabs-release-dockers:
	if [ -z $(shell docker images -q casperlabs/node:${CL_DOCKER_TAG}) ] ||             \
	   [ -z $(shell docker images -q casperlabs/execution-engine:${CL_DOCKER_TAG}) ] || \
	   [ -z $(shell docker images -q casperlabs/client:${CL_DOCKER_TAG}) ]; then        \
		echo "[x] Downloading Docker images with tag ${CL_DOCKER_TAG}..."             ; \
		docker pull casperlabs/node:${CL_DOCKER_TAG}                                  ; \
		docker pull casperlabs/execution-engine:${CL_DOCKER_TAG}                      ; \
		docker pull casperlabs/client:${CL_DOCKER_TAG}                                ; \
	fi
	echo "[x] Docker images with tag: ${CL_DOCKER_TAG}... present."

# Build test Docker images based on release images.
build-test-docker-images:
	if [ -z $(shell docker images -q casperlabs/node:test) ] ||                                    \
	   [ -z $(shell docker images -q casperlabs/execution-engine:test) ] ||                        \
	   [ -z $(shell docker images -q casperlabs/client:test) ]; then                               \
		echo "[x] Locally building docker test images..."                                        ; \
		docker tag casperlabs/execution-engine:${CL_DOCKER_TAG} casperlabs/execution-engine:test ; \
		docker tag casperlabs/client:${CL_DOCKER_TAG} casperlabs/client:test                     ; \
		if [ ! -f target/system-contracts.tar.gz ]; then                                           \
			echo "[x] Downloading 'system-contracts.tar.gz'."                                    ; \
			curl http://repo.casperlabs.io/casperlabs/repo/${CL_GIT_TAG}/system-contracts.tar.gz   \
				-o target/system-contracts.tar.gz                                                ; \
		fi                                                                                       ; \
		mkdir -p ${CL_REPO_DIR}/hack/docker/.genesis/system-contracts                            ; \
		tar -xzf target/system-contracts.tar.gz                                                    \
			-C ${CL_REPO_DIR}/hack/docker/.genesis/system-contracts                              ; \
		docker build -f ${CL_REPO_DIR}/hack/docker/test-node.Dockerfile                            \
			-t casperlabs/node:test ${CL_REPO_DIR}/hack/docker                                   ; \
		rm -rf ${CL_REPO_DIR}/hack/docker/.genesis                                               ; \
	fi
	echo "[x] Docker images with tag: test... present."

# Prepare Python environment inside CasperLabs/integration-testing directory.
build-python-environment:
	if [ ! -f ${PIPENV_LOCK_FILE} ]; then                                               \
		echo "[x] Installing Python environment."                                     ; \
		cd ${IT} && pipenv install --python python3                                   ; \
		cd ${IT} && pipenv lock -r > ${PIPENV_LOCK_FILE}                              ; \
		cd ${IT} && pipenv run client/CasperLabsClient/install.sh                     ; \
	fi
	echo "[x] Python environment is ready."

# Compile wasm file required by `casperlabs_local_net`.
build-test-wasm-contracts:
	${IT}/contracts/build_contracts.sh
	echo "[x] WASM files required by casperlabs_local_net are ready."

# Copy test and wasm files into CasperLabs repository.
reload:
	rm -f ${IT}/test/test_*.py
	cp tests/test_*.py ${IT}/test
	cargo build --release
	cp target/wasm32-unknown-unknown/release/*.wasm ${IT}/resources

# Start interactive console.
console: clean-docker-artifacts
	cp cl_test_runner.py ${IT}
	cp cl_test_context.py ${IT}
	cd ${IT} && pipenv run python -i cl_test_runner.py

# Stop running Docker nodes with node and execution-engine. Remove network.
clean-docker-artifacts:
	cd ${IT} && pipenv run python docker_cleanup_assurance.py

