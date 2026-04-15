#
# __  __       _         __ _ _
#|  \/  | __ _| | _____ / _(_) | ___
#| |\/| |/ _` | |/ / _ \ |_| | |/ _ \
#| |  | | (_| |   <  __/  _| | |  __/
#|_|  |_|\__,_|_|\_\___|_| |_|_|\___|
#
# NOTE: For more information on Kernel Module Makefile, see:
# 	https://www.kernel.org/doc/html/v5.19/kbuild/makefiles.html#linux-kernel-makefiles
#
# Makefile for building the quantis-chip-pcie kernel module

### Variables - Project
KDRIVER_NAME	:=quantis-chip-pcie
KVER		?= $(shell uname -r)
KDIR		?= /lib/modules/$(KVER)/build

### BUILD via kbuild
ifneq ($(KERNELRELEASE),)

include Kbuild

### BUILD via Makefile
else

EXTRA_CFLAGS += -Wall

### TARGET - all
all:
	$(MAKE) -C $(KDIR) M=$$PWD modules
	mkdir -p ../bin
	cp $(KDRIVER_NAME).ko ../bin/

### TARGET - info
info:
	@echo "[INFO]: Building '$(KDRIVER_NAME)' with the following config:"
	@echo -e "\t -> KERNELRELEASE: $(KERNELRELEASE)"
	@echo -e "\t -> KDIR: $(KDIR)"
	@echo -e "\t -> PWD: $$PWD"
	@echo -e "\t -> CC: $(shell cc -dumpversion)"

### TARGET - install
install: all
	$(MAKE) -C $(KDIR) M=$$PWD modules_install
	depmod -a

### TARGET - uninstall
uninstall:
	rm -f /lib/modules/$(KVER)/extra/$(KDRIVER_NAME).ko

### TARGET - clean
clean: mrproper
	$(MAKE) -C $(KDIR) M=$$PWD clean

mrproper:
	@echo "[INFO] target->mrproper"
	@echo -e "\t PWD:$$PWD"
	$(RM) *.o.ur-safe
	$(RM) -r ../bin

help:
	$(MAKE) -C $(KDIR) M=$$PWD help

endif

.PHONY: all install clean help
