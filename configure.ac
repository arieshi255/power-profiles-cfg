AC_INIT([power-profiles-cfg], [0.1], [skyliegirl33])

# Default value for prefix
prefix_temp=$prefix
exec_prefix_temp=$exec_prefix

test "${prefix}" = "NONE" && prefix="${ac_default_prefix}"
test "${exec_prefix}" = "NONE" && exec_prefix='${prefix}'

# Initial value is $exec_prefix/libexec
libexectemp="${libexecdir}"
mantemp="${mandir}"

# Expands to $prefix/libexec
eval libexectemp=\"${libexectemp}\"
eval mantemp=\"${mantemp}\"
# Expands fully
eval libexectemp=\"${libexectemp}\"
eval mantemp=\"${mantemp}\"
LIBEXECDIR=${libexectemp}
MANDIR=${mantemp}

AC_SUBST(LIBEXECDIR)
AC_SUBST(MANDIR)

AC_CONFIG_FILES([
  Makefile
  power-profiles-cfg.service
  01power-profiles-cfg
  power-profiles-cfg-sleep.sh
])

AC_OUTPUT