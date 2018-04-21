MAKE = make

SUBDIRS = kinetis-common k64

CLEANDIRS = $(SUBDIRS:%=clean-%)

.PHONY: build test clean $(SUBDIRS) $(TESTDIRS) $(CLEANDIRS)

build: $(SUBDIRS)

$(SUBDIRS):
	$(MAKE) -C $@

clean: $(CLEANDIRS)

$(CLEANDIRS):
	$(MAKE) -C $(@:clean-%=%) clean