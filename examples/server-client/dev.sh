#!/bin/bash

(cd server ; RUST_LOG=info cargo watch -x 'run') &
(cd client ; ./run.sh)

