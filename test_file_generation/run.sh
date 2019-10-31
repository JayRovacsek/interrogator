#!/bin/sh

if [ -d "./venv" ]; then
    python3 generate_test_files_from_gha.py
    exit 0
else
	if which pip3 >/dev/null; then
        echo 'Pip3 found, installing virtualenv'
        pip3 install virtualenv
        if which realpath >/dev/null; then
            REAL_PATH=$(realpath .)
            echo "${REAL_PATH}/venv"
            virtualenv "${REAL_PATH}/venv"
            source "${REAL_PATH}/venv/bin/activate"
            pip3 install -r requirements.txt
            python3 generate_test_files_from_gha.py
            exit 0
        else
            echo "Realpath not found"
            exit 1
        fi
    else
        echo 'Pip3 not found!'
        exit 1
    fi
fi