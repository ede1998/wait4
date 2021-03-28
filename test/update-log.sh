#!/bin/bash

array=(
	"File deleted."
	"User ede logged in"
	"User failed to log in"
	"User logged out"
	"Password changed successfully"
	"Email verified"
	"[2021-03-27 14:03:34] AppID 10242 state changed : Fully Installed,"
	"[2021-03-27 14:03:34] AppID 1337 state changed : Fully Installed,"
	"[2021-03-27 14:03:34] AppID 01381 state changed : Fully Installed,"
	"[2021-03-27 14:03:34] AppID 99999999 state changed : Fully Installed,"
)

rand-choice() {
    size=${#array[@]};
    index=$(($RANDOM % $size))
    echo ${array[$index]}
}

file=log.txt

while true; do
	log_line="[`date`] `rand-choice`"
	echo "$log_line" >> log.txt;
	sleep 2
done
