CHIP=bobbin-chip
PERIPH_ARGS=--periph --cargo-template template

.PHONY: build hoperf

build: hoperf

hoperf: rfm69 rfm95

rfm69:
	$(CHIP) $(PERIPH_ARGS) hoperf/$@.rx ../periph/$@/

rfm95:
	$(CHIP) $(PERIPH_ARGS) hoperf/$@.rx ../periph/$@/
