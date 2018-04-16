CHIP=bobbin-chip
CHIP_ARGS="--build"
STM32_ARGS=--crate --cargo-template bobbin-stm32/template
KINETIS_ARGS=--crate --cargo-template bobbin-kinetis/template
SAM_ARGS=--crate --cargo-template bobbin-sam/template

.PHONY: build bobbin-cortexm bobbin-kinetis bobbin-sam bobbin-stm32 bobbin-ti

build: bobbin-cortexm bobbin-kinetis bobbin-sam bobbin-stm32 bobbin-ti

bobbin-kinetis: kinetis-common k20 k64 kl26 s32

bobbin-sam: sam-common samd21

bobbin-stm32: stm32-common stm32f10x stm32f20x stm32f40x stm32f42x stm32f302x stm32f303x stm32l031x stm32l432x stm32l4x2 stm32f74x

bobbin-ti: tm4c129x

bobbin-ambiq: apollo2

bobbin-silabs: efm32pg12

bobbin-cortexm:
	$(CHIP) --crate --cargo-template template $@/lib.rx ../mcu/$@/
	
kinetis-common:
	$(CHIP) $(CHIP_ARGS) bobbin-kinetis/$@/mod.rx ../mcu/bobbin-kinetis/$@/src/periph/mod.rs

k20:
	$(CHIP) $(KINETIS_ARGS) bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/

k64:
	$(CHIP) $(KINETIS_ARGS) bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/

kl26:
	$(CHIP) $(KINETIS_ARGS) bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/

s32:
	$(CHIP) $(KINETIS_ARGS) bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/

sam-common:
	$(CHIP) $(CHIP_ARGS) bobbin-sam/$@/mod.rx ../mcu/bobbin-sam/$@/src/periph/mod.rs

samd21:
	$(CHIP) $(SAM_ARGS) bobbin-sam/$@/lib.rx ../mcu/bobbin-sam/$@/

stm32-common:
	$(CHIP) $(CHIP_ARGS) bobbin-stm32/$@/mod.rx ../mcu/bobbin-stm32/$@/src/periph/mod.rs

stm32f10x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f20x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f40x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f42x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f302x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f303x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32l031x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32l432x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32l4x2:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f74x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

tm4c129x:
	$(CHIP) $(CHIP_ARGS) bobbin-ti/$@/lib.rx ../mcu/bobbin-ti/$@/src/periph/mod.rs
	
apollo2:
	$(CHIP) $(CHIP_ARGS) bobbin-ambiq/$@/lib.rx ../mcu/bobbin-ambiq/$@/src/periph/mod.rs

efm32pg12:
	$(CHIP) $(CHIP_ARGS) bobbin-silabs/$@/lib.rx ../mcu/bobbin-silabs/$@/src/periph/mod.rs
