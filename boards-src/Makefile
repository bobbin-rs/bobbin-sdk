CHIP=bobbin-chip
CHIP_ARGS=--board --cargo-template template/

all: nucleo-l432kc nucleo-f746zg

nucleo-l432kc: nucleo-l432kc.rx
	$(CHIP) $(CHIP_ARGS) $< ../boards-new/$@/

nucleo-f746zg: nucleo-f746zg.rx
	$(CHIP) $(CHIP_ARGS) $< ../boards-new/$@/