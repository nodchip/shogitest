.RECIPEPREFIX = >
SUFFIX :=

ifeq ($(OS),Windows_NT)
    EXE ?= shogitest.exe
else
    EXE ?= shogitest
endif

all:
> @echo "This Makefile is intended for use by OpenBench / ShogiBench only."
> @exit 1

openbench:
> cargo rustc --release -- -C target-cpu=native --emit link=$(EXE)

.PHONY: all openbench
