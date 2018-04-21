MAKE = make

SUBDIRS = stm32-common stm32f74x stm32f303x stm32l432x

CLEANDIRS = $(SUBDIRS:%=clean-%)

.PHONY: build test clean $(SUBDIRS) $(TESTDIRS) $(CLEANDIRS)

build: $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@

clean: $(CLEANDIRS)

$(CLEANDIRS):
	$(MAKE) -C $(@:clean-%=%) clean