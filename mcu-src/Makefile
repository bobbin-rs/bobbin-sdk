CHIP=bobbin-chip
CHIP_ARGS="--build"
STM32_ARGS=--crate --cargo-template bobbin-stm32/template
KINETIS_ARGS=--crate --cargo-template bobbin-kinetis/template
SAM_ARGS=--crate --cargo-template bobbin-sam/template

.PHONY: build bobbin-cortexm bobbin-kinetis bobbin-sam bobbin-stm32

build: bobbin-cortexm bobbin-kinetis bobbin-sam bobbin-stm32

bobbin-kinetis: kinetis-common k64

bobbin-sam: sam-common samd21 samd51

bobbin-stm32: stm32-common stm32f303x stm32f40x stm32f42x stm32l432x stm32f74x

bobbin-cortexm:
	$(CHIP) --crate --cargo-template bobbin-cortexm/template $@/lib.rx ../mcu/$@/
	
kinetis-common:
	$(CHIP) --crate --cargo-template bobbin-kinetis/kinetis-common/template bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/

k64:
	$(CHIP) $(KINETIS_ARGS) bobbin-kinetis/$@/lib.rx ../mcu/bobbin-kinetis/$@/

sam-common:
	$(CHIP) --crate --cargo-template bobbin-sam/sam-common/template bobbin-sam/$@/lib.rx ../mcu/bobbin-sam/$@/

samd21:
	$(CHIP) $(SAM_ARGS) bobbin-sam/$@/lib.rx ../mcu/bobbin-sam/$@/

samd51:
	$(CHIP) $(SAM_ARGS) bobbin-sam/$@/lib.rx ../mcu/bobbin-sam/$@/

stm32-common:
	$(CHIP) --crate --cargo-template bobbin-stm32/stm32-common/template bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f303x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f40x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f42x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32l432x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/

stm32f74x:
	$(CHIP) $(STM32_ARGS) bobbin-stm32/$@/lib.rx ../mcu/bobbin-stm32/$@/
