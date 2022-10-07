#!/bin/sh

bindgen iperf_api.h  \
--allowlist-function 'iperf_*|set_protocol|get_protocol' \
--allowlist-type 'iperf_*' \
--blocklist-function 'iperf_(send|recv|get_test_outfile)' \
--blocklist-type 'FILE|fd_set|__sbuf|__sFILE|__sFILEX' \
-o src/iperf_bindings.rs \
-- -D HAVE_STDINT_H