#!/bin/sh

JLinkExe -if SWD -device S32K144 -autoconnect 1 -speed 4000 $*