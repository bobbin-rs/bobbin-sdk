.PHONY: build arduino-zero discovery-stm32f4 ek-tm4c1294xl evb-s32k144 feather-m0 frdm-k64f \
	frdm-kl26z nucleo-f031k6 nucleo-f207zg nucleo-f302r8 nucleo-f303ze nucleo-f429zi \
	teensy-32 teensy-35 teensy-36 teensy-lc

build: arduino-zero discovery-stm32f4 ek-tm4c1294xl evb-s32k144 feather-m0 frdm-k64f \
	frdm-kl26z nucleo-f031k6 nucleo-f207zg nucleo-f302r8 nucleo-f303ze nucleo-f429zi \
	teensy-32 teensy-35 teensy-36 teensy-lc

arduino-zero:
	cd arduino-zero && bobbin build

discovery-stm32f4:
	cd discovery-stm32f4 && bobbin build

ek-tm4c1294xl:
	cd ek-tm4c1294xl && bobbin build

evb-s32k144:
	cd evb-s32k144 && bobbin build

feather-m0:
	cd feather-m0 && bobbin build

frdm-k64f:
	cd frdm-k64f && bobbin build

frdm-kl26z:
	cd frdm-kl26z && bobbin build

nucleo-f031k6:
	cd nucleo-f031k6 && bobbin build

nucleo-f207zg:
	cd nucleo-f207zg && bobbin build

nucleo-f302r8:
	cd nucleo-f302r8 && bobbin build

nucleo-f303ze:
	cd nucleo-f303ze && bobbin build

nucleo-f429zi:
	cd nucleo-f429zi && bobbin build

teensy-32:
	cd teensy-32 && bobbin build

teensy-35:
	cd teensy-35 && bobbin build

teensy-36:
	cd teensy-36 && bobbin build

teensy-lc:
	cd teensy-lc && bobbin build
