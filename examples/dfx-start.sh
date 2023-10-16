#!/usr/bin/env bash

if [[ $* == *--use-old-metering* ]]; then
    dfx start --use-old-metering --clean --background
else
    dfx start --clean --background
fi

# if --vetkd is specified, deploy vetkd_system_api
if [[ $* == *--vetkd* ]]; then
    dfx deploy vetkd_system_api --specified-id wfdtj-lyaaa-aaaap-abakq-cai
fi
