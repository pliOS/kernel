# Copyright 2016 The pliOS Developers. See the LICENSE
# file at the top-level directory of this distribution.
#
# Licensed under the MIT license <LICENSE-MIT or
# http://opensource.org/licenses/MIT>. This file may not
# be copied, modified, or distributed except according
# to these terms.

.PHONY: all clean run cargo

ARCH ?= x86_64
OUT_DIR := out
LINK_SCRIPT := src/$(ARCH)/boot/kernel.ld

TARGET := $(ARCH)-unknown-none
RUST_OS := target/$(TARGET)/debug/libplios_kernel.a

export RUST_TARGET_PATH := $(shell pwd)/targets/

include src/$(ARCH)/boot/Rules.mk

all: $(OUT_DIR)/os.iso

clean:
	@rm -r $(OUT_DIR)

run: $(OUT_DIR)/os.iso
	@qemu-system-$(ARCH) -cdrom $(OUT_DIR)/os.iso -nographic

cargo:
	@xargo build --target $(TARGET)

$(OUT_DIR)/os.iso: $(OUT_DIR)/plikernel share/grub.cfg
	@echo "MK $@"
	@mkdir -p $(OUT_DIR)/root/boot/grub
	@cp $(OUT_DIR)/plikernel $(OUT_DIR)/root/boot
	@cp share/grub.cfg $(OUT_DIR)/root/boot/grub
	@grub-mkrescue -o $(OUT_DIR)/os.iso out/root 2> /dev/null
	@rm -r out/root

out/plikernel: $(OBJECT_FILES) $(LINK_SCRIPT) cargo
	@echo "LD $@"
	@$(LD) --gc-sections -T $(LINK_SCRIPT) -o out/plikernel $(OBJECT_FILES) $(RUST_OS)

$(OUT_DIR)/%.o: %.S
	@echo "AS $<"
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
