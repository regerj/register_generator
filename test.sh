#!/bin/bash

# Some locations
PROJECT_DIR=${PWD}/
PROJECT_BINARY=${PROJECT_DIR}target/debug/register_generator
TEST_DIR=${PROJECT_DIR}test/
BUILD_DIR=${TEST_DIR}build/
TEST_BINARY=${BUILD_DIR}src/test
CFG_DIR=${TEST_DIR}cfg/
ENV_DIR=${TEST_DIR}environment/

# Colours
RED="\e[31m"
GREEN="\e[32m"
BOLDGREEN="\e[1;32m"
ENDCOLOR="\e[0m"

# Initial setup for test
cargo build
mkdir ${ENV_DIR}
cp ${PROJECT_BINARY} ${ENV_DIR}
cd ${ENV_DIR}

# Generate our headers
for filename in ${CFG_DIR}*; do
    ./register_generator generate --path $filename
done

# Build the test binary
cmake ${BUILD_DIR}..
cd ${BUILD_DIR}
make
cd ${ENV_DIR}

# Run the test binary
${TEST_BINARY}

echo -e "${BOLDGREEN}SUCCESS${ENDCOLOR}"

# Teardown the environment
cd ..
rm -rf ./environment
