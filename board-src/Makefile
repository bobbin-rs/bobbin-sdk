.PHONY: all

CHIP=bobbin-chip
CHIP_ARGS_THUMBV6=--board --cargo-template template-thumbv6/
CHIP_ARGS_THUMBV7=--board --cargo-template template-thumbv7/
BOARD_DST=../board

all: arduino-zero frdm-k64f nucleo-l432kc nucleo-f746zg discovery-stm32f3

arduino-zero: arduino-zero.rx
	$(CHIP) $(CHIP_ARGS_THUMBV6) $< $(BOARD_DST)/$@/

frdm-k64f: frdm-k64f.rx
	$(CHIP) $(CHIP_ARGS_THUMBV7) $< $(BOARD_DST)/$@/

nucleo-l432kc: nucleo-l432kc.rx
	$(CHIP) $(CHIP_ARGS_THUMBV7) $< $(BOARD_DST)/$@/

nucleo-f746zg: nucleo-f746zg.rx
	$(CHIP) $(CHIP_ARGS_THUMBV7) $< $(BOARD_DST)/$@/

discovery-stm32f3: discovery-stm32f3.rx
	$(CHIP) $(CHIP_ARGS_THUMBV7) $< $(BOARD_DST)/$@/