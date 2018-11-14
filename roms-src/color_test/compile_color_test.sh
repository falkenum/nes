#!/bin/bash
ca65 color_test.s -g -o temp/color_test.o
ld65 -C nrom.cfg -o ../../color_test.nes temp/color_test.o -m temp/color_test_map.txt -Ln temp/color_test_labels.txt
