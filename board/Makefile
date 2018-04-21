MAKE = make

SUBDIRS = arduino-zero frdm-k64f nucleo-f746zg nucleo-l432kc discovery-stm32f3

TESTDIRS = test-arduino-zero test-discovery-stm32f4 test-ek-tm4c1294xl test-evb-s32k144 test-frdm-k64f \
	test-frdm-kl26z test-nucleo-l031k6 test-nucleo-f103rb test-nucleo-f207zg test-nucleo-f302r8 \
	test-nucleo-f303re test-nucleo-f429zi test-nucleo-l432kc test-silabs-slstk3401a test-amaph1kk-kbr

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