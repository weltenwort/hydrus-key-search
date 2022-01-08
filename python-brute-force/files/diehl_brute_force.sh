#/bin/sh

for i in $(awk 'BEGIN { for ( i=0; i<10000; i++ ) { print i; } }'); do sleep 5; dmes
