#!/bin/bash

if ! adb install -r /home/albin/Documents/amp/target/dx/amp/release/android/app/app/build/outputs/apk/debug/app-debug.apk; then
  adb install -r /home/albin/Documents/amp/target/dx/amp/release/android/app/app/build/outputs/apk/release/app-release.apk;
fi