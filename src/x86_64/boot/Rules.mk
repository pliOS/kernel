# Copyright 2016 The pliOS Developers. See the LICENSE
# file at the top-level directory of this distribution.
#
# Licensed under the MIT license <LICENSE-MIT or
# http://opensource.org/licenses/MIT>. This file may not
# be copied, modified, or distributed except according
# to these terms.

LD := x86_64-elf-ld.gold
AS := nasm

ASFLAGS := -felf64

SRC_DIR := src/x86_64/boot
ASM_SOURCES := boot.S boot64.S

OBJECT_FILES := $(patsubst %.S, $(OUT_DIR)/$(SRC_DIR)/%.o, $(ASM_SOURCES))
