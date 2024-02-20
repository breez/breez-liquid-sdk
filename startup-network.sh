#! /bin/bash

set -e

bitcoind > /dev/null &

lightningd --conf=/breez-liquid/cfg/swapper-ln-config &
lightningd --conf=/breez-liquid/cfg/user-ln-config &

# Run an instance of bash so we can keep the container running
bash
