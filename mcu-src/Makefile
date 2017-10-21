CHIP=bobbin-chip
CHIP_ARGS="--build"

.PHONY: build bobbin-cortexm bobbin-kinetis bobbin-sam bobbin-stm32 bobbin-ti

build: bobbin-cortexm bobbin-kinetis bobbin-sam bobbin-stm32 bobbin-ti

bobbin-kinetis: kinetis-common k20 k64 kl26 s32

bobbin-sam: samd21

bobbin-stm32: stm32-common stm32f10x stm32f20x stm32f40x stm32f42x stm32f302x stm32f303x stm32l031x stm32l432x stm32f74x

bobbin-ti: tm4c129x

bobbin-ambiq: apollo2

bobbin-silabs: efm32pg12

bobbin-cortexm:
	$(CHIP) $(CHIP_ARGS) $@/mod.rx ../mcu/$@/src/chip/mod.rs
	
kinetis-common:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/mod.rx ../mcu/bobbin-kinetis/$@/src/chip/mod.rs

k20:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/src/chip/mod.rs

k64:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/src/chip/mod.rs

kl26:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/src/chip/mod.rs

s32:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/src/chip/mod.rs

samd21:
	$(CHIP) $(CHIP_ARGS) bobbin-sam/$@/lib.rx ../mcu/bobbin-sam/$@/src/chip/mod.rs

stm32-common:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/mod.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f10x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f20x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f40x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f42x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f302x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f303x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32l031x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32l432x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

stm32f74x:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/src/chip/mod.rs

tm4c129x:
	$(CHIP) $(CHIP_ARGS) bobbin-ti/$@/lib.rx ../mcu/bobbin-ti/$@/src/chip/mod.rs
	
apollo2:
	$(CHIP) $(CHIP_ARGS) bobbin-ambiq/$@/lib.rx ../mcu/bobbin-ambiq/$@/src/chip/mod.rs

efm32pg12:
	$(CHIP) $(CHIP_ARGS) bobbin-silabs/$@/lib.rx ../mcu/bobbin-silabs/$@/src/chip/mod.rs
