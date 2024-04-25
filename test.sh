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
BOLDRED="\e[1;31m"
ENDCOLOR="\e[0m"

# Initial setup for test
cargo build
RESULT=$?
if [ ${RESULT} -ne 0 ]
then
    echo -e "${BOLDRED}CARGO BUILD FAIL${ENDCOLOR}"
    exit 1
fi

mkdir ${ENV_DIR}
cp ${PROJECT_BINARY} ${ENV_DIR}
cd ${ENV_DIR}

# Generate our headers
for filename in ${CFG_DIR}*; do
    ./register_generator generate --path $filename
    RESULT=$?
    if [ ${RESULT} -ne 0 ]
    then
        echo -e "${BOLDRED}HEADER GENERATION FAIL${ENDCOLOR}"
        exit 1
    fi
done

# Create the build directory if it doesn't exist
if [ ! -d ${BUILD_DIR} ]
then
    mkdir ${BUILD_DIR}
fi

cd ${BUILD_DIR}

# Generate makefiles
cmake ..
RESULT=$?
if [ ${RESULT} -ne 0 ]
then
    echo -e "${BOLDRED}CMAKE GENERATION FAIL${ENDCOLOR}"
    exit 1
fi

# Build the test binary
make
RESULT=$?
if [ ${RESULT} -ne 0 ]
then
    echo -e "${BOLDRED}BUILD TEST BINARY FAIL${ENDCOLOR}"
    exit 1
fi

cd ${ENV_DIR}

# Run the test binary
${TEST_BINARY} --gtest_output=xml:test-report.xml
RESULT=$?
if [ ${RESULT} -eq 0 ]
then
    echo -e "${BOLDGREEN}SUCCESS${ENDCOLOR}"
    exit 0
else
    echo -e "${BOLDRED}TEST FAIL${ENDCOLOR}"
    exit 1
fi
