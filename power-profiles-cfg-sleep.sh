#!/bin/sh
#
# power-profiles-cfg: Re-apply power profile settings

case $1/$2 in
  pre/*)
    # Stopping is not required.
    ;;
  post/*)
    # Make power-profiles-cfg forcibly re-apply the profile settings.
    if [ -e /usr/lib/power-profiles-cfg ] ; then
      /usr/lib/power-profiles-cfg --force
    fi
    ;;
esac