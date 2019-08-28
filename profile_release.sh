#!/usr/bin/env sh

time flamegraph -o "profile/release_profile.svg" -- ./target/release/interrogator $1 -k $2 -r $3 -o $4 -vs -g $5
chown jrovacsek:everyone "profile/release_profile.svg"
