AC_INIT([power-profiles-cfg], [0.1], [skyliegirl33])

# Default value for prefix
prefix_temp=$prefix
exec_prefix_temp=$exec_prefix

test "${prefix}" = "NONE" && prefix="${ac_default_prefix}"
test "${exec_prefix}" = "NONE" && exec_prefix='${prefix}'

# Initial value is $exec_prefix/sbin
sbintemp="${sbindir}"
mantemp="${mandir}"

# Expands to $prefix/sbin
eval sbintemp=\"${sbintemp}\"
eval mantemp=\"${mantemp}\"
# Expands fully
eval sbintemp=\"${sbintemp}\"
eval mantemp=\"${mantemp}\"
SBINDIR=${sbintemp}
MANDIR=${mantemp}

AC_SUBST(SBINDIR)
AC_SUBST(MANDIR)

AC_CONFIG_FILES([
  Makefile
  power-profiles-cfg.service
])

AC_OUTPUT