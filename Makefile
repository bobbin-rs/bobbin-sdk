CHIP=bobbin-chip
CHIP_ARGS="--build"

.PHONY: build bobbin-cortexm bobbin-kinetis bobbin-sam bobbin-stm32 bobbin-ti

build: bobbin-cortexm bobbin-kinetis bobbin-sam bobbin-stm32 bobbin-ti

bobbin-kinetis: kinetis-common k20 k64 kl26 s32

bobbin-sam: samd21

bobbin-stm32: stm32-common stm32f10x stm32f20x stm32f40x stm32f42x stm32f302x stm32f303x stm32l031x stm32l432x

bobbin-ti: tm4c129x

bobbin-ambiq: apollo2

bobbin-cortexm:
	$(CHIP) $(CHIP_ARGS) $@/mod.rx ../bobbin-mcu/$@/src/chip/mod.rs
	
kinetis-common:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/mod.rx ../bobbin-mcu/bobbin-kinetis/$@/src/chip/mod.rs

k20:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/lib.rx ../bobbin-mcu/bobbin-kinetis/$@/src/chip/mod.rs

k64:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/lib.rx ../bobbin-mcu/bobbin-kinetis/$@/src/chip/mod.rs

kl26:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/lib.rx ../bobbin-mcu/bobbin-kinetis/$@/src/chip/mod.rs

s32:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/lib.rx ../bobbin-mcu/bobbin-kinetis/$@/src/chip/mod.rs

samd21:
	$(CHIP) $(CHIP_ARGS) bobbin-sam/$@/lib.rx ../bobbin-mcu/bobbin-sam/$@/src/chip/mod.rs

stm32-common:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/mod.rx ../bobbin-mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f10x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../bobbin-mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f20x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../bobbin-mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f40x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../bobbin-mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f42x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../bobbin-mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f302x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../bobbin-mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f303x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../bobbin-mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32l031x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../bobbin-mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32l432x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../bobbin-mcu/bobbin-stm32/$@/src/chip/mod.rs

tm4c129x:
	$(CHIP) $(CHIP_ARGS) bobbin-ti/$@/lib.rx ../bobbin-mcu/bobbin-ti/$@/src/chip/mod.rs
	
apollo2:
	$(CHIP) $(CHIP_ARGS) bobbin-ambiq/$@/lib.rx ../bobbin-mcu/bobbin-ambiq/$@/src/chip/mod.rs
