#!/usr/bin/env sh

time flamegraph -o "profile/development_profile.svg" -- ./target/debug/interrogator $1 -k $2 -r $3 -o $4 -vs -g $5
chown jrovacsek:everyone "profile/development_profile.svg"
