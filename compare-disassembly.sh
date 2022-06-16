#!/bin/sh

strip_assembly ()
{
    perl -pe 's/^ *[0-9a-f]+://; s/;.*$//' "$@"
}

cargo build --release &&
avr-objdump -d target/avr-atmega328p/release/bad.elf  | tee /tmp/bad.txt | strip_assembly > /tmp/bad.noaddr.txt &&
avr-objdump -d target/avr-atmega328p/release/good.elf  | tee /tmp/good.txt | strip_assembly > /tmp/good.noaddr.txt &&
diff -uw /tmp/bad.noaddr.txt /tmp/good-noaddr.txt
