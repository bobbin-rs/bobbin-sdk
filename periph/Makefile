MAKE = make

SUBDIRS = rfm69 rfm95 cortex-m

TESTDIRS = $(SUBDIRS:%=test-%)

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