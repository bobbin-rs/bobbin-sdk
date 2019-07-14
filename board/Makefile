MAKE = make

SUBDIRS = arduino-zero discovery-stm32f3 discovery-stm32f429i feather-m0 nucleo-f429zi nucleo-f746zg nucleo-l432kc

TESTDIRS = test-arduino-zero test-frdm-k64f test-nucleo-f746zg test-nucleo-l432kc test-discovery-stm32f3

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