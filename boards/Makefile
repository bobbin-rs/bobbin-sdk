MAKE = make

SUBDIRS = amaph1kk-kbr arduino-zero discovery-stm32f4 ek-tm4c1294xl evb-s32k144 feather-m0 frdm-k64f \
	frdm-kl26z nucleo-l031k6 nucleo-f103rb nucleo-f207zg nucleo-f302r8 nucleo-f303re nucleo-f429zi \
	nucleo-l432kc silabs-slstk3401a teensy-32 teensy-35 teensy-36 teensy-lc

TESTDIRS = test-arduino-zero test-discovery-stm32f4 test-ek-tm4c1294xl test-evb-s32k144 test-frdm-k64f \
	test-frdm-kl26z test-nucleo-l031k6 test-nucleo-f103rb test-nucleo-f207zg test-nucleo-f302r8 \
	test-nucleo-f303re test-nucleo-f429zi test-nucleo-l432kc

CLEANDIRS = $(SUBDIRS:%=clean-%)

.PHONY: build test clean $(SUBDIRS) $(TESTDIRS) $(CLEANDIRS)

build: $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@

test: $(TESTDIRS)

$(TESTDIRS): 
	$(MAKE) -C $(@:test-%=%) test

clean: $(CLEANDIRS)

$(CLEANDIRS):
	$(MAKE) -C $(@:clean-%=%) clean