# hydrus-key-search

:warning: This has not been cleaned up for general consumption. Use at your own risk.

## Contents

* `generate-oms-key-candidates` contains a key generator that exhaustively searches a key space using all cpu cores. It's unfinished and can only search symmetric half-keys so far.
* `python-brute-force` contains the first attempt to try out keys and use the actual https://github.com/weetmuts/wmbusmeters simulation mode as an oracle before I re-implemented the decryption in rust.
