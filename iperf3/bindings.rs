mod iperf_bindings {
    #[repr(C)]
    pub struct iperf_test {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_test {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_test",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_test {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_test {
        #[inline]
        fn clone(&self) -> iperf_test {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct protocol {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for protocol {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "protocol",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for protocol {}
    #[automatically_derived]
    impl ::core::clone::Clone for protocol {
        #[inline]
        fn clone(&self) -> protocol {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    extern "C" {
        pub fn get_protocol(arg1: *mut iperf_test, arg2: ::std::os::raw::c_int) -> *mut protocol;
    }
    extern "C" {
        pub fn set_protocol(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
}
mod iperf_bindings {
    #[repr(C)]
    pub struct iperf_test {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_test {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_test",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_test {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_test {
        #[inline]
        fn clone(&self) -> iperf_test {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct protocol {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for protocol {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "protocol",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for protocol {}
    #[automatically_derived]
    impl ::core::clone::Clone for protocol {
        #[inline]
        fn clone(&self) -> protocol {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    extern "C" {
        pub fn get_protocol(arg1: *mut iperf_test, arg2: ::std::os::raw::c_int) -> *mut protocol;
    }
    extern "C" {
        pub fn set_protocol(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
}
mod iperf_bindings {
    #[repr(C)]
    pub struct iperf_test {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_test {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_test",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_test {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_test {
        #[inline]
        fn clone(&self) -> iperf_test {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct protocol {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for protocol {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "protocol",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for protocol {}
    #[automatically_derived]
    impl ::core::clone::Clone for protocol {
        #[inline]
        fn clone(&self) -> protocol {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    extern "C" {
        pub fn get_protocol(arg1: *mut iperf_test, arg2: ::std::os::raw::c_int) -> *mut protocol;
    }
    extern "C" {
        pub fn set_protocol(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
}
mod iperf_bindings {
    pub type size_t = ::std::os::raw::c_ulong;
    pub type __uint64_t = ::std::os::raw::c_ulong;
    pub type __off_t = ::std::os::raw::c_long;
    pub type __off64_t = ::std::os::raw::c_long;
    pub type __fd_mask = ::std::os::raw::c_long;
    #[repr(C)]
    pub struct _IO_marker {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_marker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_marker",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_marker {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_marker {
        #[inline]
        fn clone(&self) -> _IO_marker {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_codecvt {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_codecvt {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_codecvt",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_codecvt {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_codecvt {
        #[inline]
        fn clone(&self) -> _IO_codecvt {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_wide_data {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_wide_data {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_wide_data",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_wide_data {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_wide_data {
        #[inline]
        fn clone(&self) -> _IO_wide_data {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type _IO_lock_t = ::std::os::raw::c_void;
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::std::os::raw::c_int,
        pub _IO_read_ptr: *mut ::std::os::raw::c_char,
        pub _IO_read_end: *mut ::std::os::raw::c_char,
        pub _IO_read_base: *mut ::std::os::raw::c_char,
        pub _IO_write_base: *mut ::std::os::raw::c_char,
        pub _IO_write_ptr: *mut ::std::os::raw::c_char,
        pub _IO_write_end: *mut ::std::os::raw::c_char,
        pub _IO_buf_base: *mut ::std::os::raw::c_char,
        pub _IO_buf_end: *mut ::std::os::raw::c_char,
        pub _IO_save_base: *mut ::std::os::raw::c_char,
        pub _IO_backup_base: *mut ::std::os::raw::c_char,
        pub _IO_save_end: *mut ::std::os::raw::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::std::os::raw::c_int,
        pub _flags2: ::std::os::raw::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: ::std::os::raw::c_ushort,
        pub _vtable_offset: ::std::os::raw::c_schar,
        pub _shortbuf: [::std::os::raw::c_char; 1usize],
        pub _lock: *mut _IO_lock_t,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::std::os::raw::c_void,
        pub __pad5: size_t,
        pub _mode: ::std::os::raw::c_int,
        pub _unused2: [::std::os::raw::c_char; 20usize],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_FILE {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "_flags",
                "_IO_read_ptr",
                "_IO_read_end",
                "_IO_read_base",
                "_IO_write_base",
                "_IO_write_ptr",
                "_IO_write_end",
                "_IO_buf_base",
                "_IO_buf_end",
                "_IO_save_base",
                "_IO_backup_base",
                "_IO_save_end",
                "_markers",
                "_chain",
                "_fileno",
                "_flags2",
                "_old_offset",
                "_cur_column",
                "_vtable_offset",
                "_shortbuf",
                "_lock",
                "_offset",
                "_codecvt",
                "_wide_data",
                "_freeres_list",
                "_freeres_buf",
                "__pad5",
                "_mode",
                "_unused2",
            ];
            let values: &[&dyn::core::fmt::Debug] = &[
                &&self._flags,
                &&self._IO_read_ptr,
                &&self._IO_read_end,
                &&self._IO_read_base,
                &&self._IO_write_base,
                &&self._IO_write_ptr,
                &&self._IO_write_end,
                &&self._IO_buf_base,
                &&self._IO_buf_end,
                &&self._IO_save_base,
                &&self._IO_backup_base,
                &&self._IO_save_end,
                &&self._markers,
                &&self._chain,
                &&self._fileno,
                &&self._flags2,
                &&self._old_offset,
                &&self._cur_column,
                &&self._vtable_offset,
                &&self._shortbuf,
                &&self._lock,
                &&self._offset,
                &&self._codecvt,
                &&self._wide_data,
                &&self._freeres_list,
                &&self._freeres_buf,
                &&self.__pad5,
                &&self._mode,
                &&self._unused2,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "_IO_FILE", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_FILE {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_FILE {
        #[inline]
        fn clone(&self) -> _IO_FILE {
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_marker>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<__off_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_ushort>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_schar>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 1usize]>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_lock_t>;
            let _: ::core::clone::AssertParamIsClone<__off64_t>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_codecvt>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_wide_data>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_void>;
            let _: ::core::clone::AssertParamIsClone<size_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 20usize]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_test {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_test {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_test",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_test {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_test {
        #[inline]
        fn clone(&self) -> iperf_test {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_stream {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_stream {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_stream",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_stream {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_stream {
        #[inline]
        fn clone(&self) -> iperf_stream {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_time {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_time {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_time",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_time {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_time {
        #[inline]
        fn clone(&self) -> iperf_time {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type iperf_size_t = u64;
    extern "C" {
        pub fn iperf_get_verbose(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_control_socket(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_omit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_duration(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_role(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_reverse(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_blksize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_rate(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_pacing_timer(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_bytes(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_blocks(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_burst(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_socket_bufsize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_reporter_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_stats_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_num_streams(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_repeating_payload(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamps(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamp_format(
            ipt: *mut iperf_test,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_server_port(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_server_hostname(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_template(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_protocol_id(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output_string(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_zerocopy(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_get_server_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_unit_format(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_bind_address(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_udp_counters_64bit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_one_off(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_tos(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_extra_data(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_iperf_version() -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_no_delay(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_connect_timeout(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_dont_fragment(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_congestion_control(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_set_verbose(ipt: *mut iperf_test, verbose: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_control_socket(ipt: *mut iperf_test, ctrl_sck: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_omit(ipt: *mut iperf_test, omit: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_duration(ipt: *mut iperf_test, duration: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_reporter_interval(ipt: *mut iperf_test, reporter_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_stats_interval(ipt: *mut iperf_test, stats_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_state(ipt: *mut iperf_test, state: ::std::os::raw::c_schar);
    }
    extern "C" {
        pub fn iperf_set_test_blksize(ipt: *mut iperf_test, blksize: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_logfile(ipt: *mut iperf_test, logfile: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_rate(ipt: *mut iperf_test, rate: u64);
    }
    extern "C" {
        pub fn iperf_set_test_pacing_timer(
            ipt: *mut iperf_test,
            pacing_timer: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bytes(ipt: *mut iperf_test, bytes: u64);
    }
    extern "C" {
        pub fn iperf_set_test_blocks(ipt: *mut iperf_test, blocks: u64);
    }
    extern "C" {
        pub fn iperf_set_test_burst(ipt: *mut iperf_test, burst: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_server_port(ipt: *mut iperf_test, server_port: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_socket_bufsize(
            ipt: *mut iperf_test,
            socket_bufsize: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_num_streams(ipt: *mut iperf_test, num_streams: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_repeating_payload(
            ipt: *mut iperf_test,
            repeating_payload: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_timestamps(ipt: *mut iperf_test, timestamps: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_timestamp_format(
            arg1: *mut iperf_test,
            tf: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_role(ipt: *mut iperf_test, role: ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_server_hostname(
            ipt: *mut iperf_test,
            server_hostname: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_template(
            ipt: *mut iperf_test,
            tmp_template: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_reverse(ipt: *mut iperf_test, reverse: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_json_output(ipt: *mut iperf_test, json_output: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_has_zerocopy() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_set_test_zerocopy(ipt: *mut iperf_test, zerocopy: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_get_server_output(
            ipt: *mut iperf_test,
            get_server_output: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_unit_format(
            ipt: *mut iperf_test,
            unit_format: ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bind_address(
            ipt: *mut iperf_test,
            bind_address: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_udp_counters_64bit(
            ipt: *mut iperf_test,
            udp_counters_64bit: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_one_off(ipt: *mut iperf_test, one_off: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_tos(ipt: *mut iperf_test, tos: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_extra_data(ipt: *mut iperf_test, dat: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_bidirectional(
            ipt: *mut iperf_test,
            bidirectional: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_no_delay(ipt: *mut iperf_test, no_delay: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_dont_fragment(ipt: *mut iperf_test, dont_fragment: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_congestion_control(
            ipt: *mut iperf_test,
            cc: *mut ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_connect_timeout(ipt: *mut iperf_test, ct: ::std::os::raw::c_int);
    }
    extern "C" {
        /// exchange_parameters - handles the param_Exchange part for client
        ///
        pub fn iperf_exchange_parameters(test: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_stats_callback -- handles the statistic gathering
        ///
        pub fn iperf_stats_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_reporter_callback -- handles the report printing
        ///
        pub fn iperf_reporter_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_test -- return a new iperf_test with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_test() -> *mut iperf_test;
    }
    extern "C" {
        pub fn iperf_defaults(testp: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_test -- free resources used by test, calls iperf_free_stream to
        /// free streams
        ///
        pub fn iperf_free_test(testp: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_stream -- return a net iperf_stream with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_stream(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> *mut iperf_stream;
    }
    extern "C" {
        /// iperf_add_stream -- add a stream to a test
        ///
        pub fn iperf_add_stream(test: *mut iperf_test, stream: *mut iperf_stream);
    }
    extern "C" {
        /// iperf_init_stream -- init resources associated with test
        ///
        pub fn iperf_init_stream(
            arg1: *mut iperf_stream,
            arg2: *mut iperf_test,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_stream -- free resources associated with test
        ///
        pub fn iperf_free_stream(sp: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_set_send_state(
            test: *mut iperf_test,
            state: ::std::os::raw::c_schar,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_throttle(sp: *mut iperf_stream, nowP: *mut iperf_time);
    }
    extern "C" {
        pub fn iperf_catch_sigend(
            handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        );
    }
    extern "C" {
        pub fn iperf_got_sigend(test: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_exchange_results(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_init_test(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_send_timers(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_parse_arguments(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_open_logfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_reset_test(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_reset_stats(test: *mut iperf_test);
    }
    #[repr(C)]
    pub struct protocol {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for protocol {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "protocol",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for protocol {}
    #[automatically_derived]
    impl ::core::clone::Clone for protocol {
        #[inline]
        fn clone(&self) -> protocol {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    extern "C" {
        pub fn get_protocol(arg1: *mut iperf_test, arg2: ::std::os::raw::c_int) -> *mut protocol;
    }
    extern "C" {
        pub fn set_protocol(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_on_new_stream(arg1: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_on_test_start(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_connect(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_test_finish(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_run_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_connect(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_streams(
            arg1: *mut iperf_test,
            sender: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_client_end(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_run_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_server_listen(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_accept(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_delete_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_total_rate(arg1: *mut iperf_test, arg2: iperf_size_t);
    }
    extern "C" {
        pub fn iperf_json_start(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_json_finish(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_setaffinity(
            arg1: *mut iperf_test,
            affinity: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_clearaffinity(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_printf(
            test: *mut iperf_test,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_err(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_errexit(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_strerror(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
}
mod iperf_bindings {
    pub type size_t = ::std::os::raw::c_ulong;
    pub type __uint64_t = ::std::os::raw::c_ulong;
    pub type __off_t = ::std::os::raw::c_long;
    pub type __off64_t = ::std::os::raw::c_long;
    pub type __fd_mask = ::std::os::raw::c_long;
    #[repr(C)]
    pub struct _IO_marker {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_marker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_marker",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_marker {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_marker {
        #[inline]
        fn clone(&self) -> _IO_marker {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_codecvt {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_codecvt {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_codecvt",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_codecvt {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_codecvt {
        #[inline]
        fn clone(&self) -> _IO_codecvt {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_wide_data {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_wide_data {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_wide_data",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_wide_data {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_wide_data {
        #[inline]
        fn clone(&self) -> _IO_wide_data {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type _IO_lock_t = ::std::os::raw::c_void;
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::std::os::raw::c_int,
        pub _IO_read_ptr: *mut ::std::os::raw::c_char,
        pub _IO_read_end: *mut ::std::os::raw::c_char,
        pub _IO_read_base: *mut ::std::os::raw::c_char,
        pub _IO_write_base: *mut ::std::os::raw::c_char,
        pub _IO_write_ptr: *mut ::std::os::raw::c_char,
        pub _IO_write_end: *mut ::std::os::raw::c_char,
        pub _IO_buf_base: *mut ::std::os::raw::c_char,
        pub _IO_buf_end: *mut ::std::os::raw::c_char,
        pub _IO_save_base: *mut ::std::os::raw::c_char,
        pub _IO_backup_base: *mut ::std::os::raw::c_char,
        pub _IO_save_end: *mut ::std::os::raw::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::std::os::raw::c_int,
        pub _flags2: ::std::os::raw::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: ::std::os::raw::c_ushort,
        pub _vtable_offset: ::std::os::raw::c_schar,
        pub _shortbuf: [::std::os::raw::c_char; 1usize],
        pub _lock: *mut _IO_lock_t,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::std::os::raw::c_void,
        pub __pad5: size_t,
        pub _mode: ::std::os::raw::c_int,
        pub _unused2: [::std::os::raw::c_char; 20usize],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_FILE {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "_flags",
                "_IO_read_ptr",
                "_IO_read_end",
                "_IO_read_base",
                "_IO_write_base",
                "_IO_write_ptr",
                "_IO_write_end",
                "_IO_buf_base",
                "_IO_buf_end",
                "_IO_save_base",
                "_IO_backup_base",
                "_IO_save_end",
                "_markers",
                "_chain",
                "_fileno",
                "_flags2",
                "_old_offset",
                "_cur_column",
                "_vtable_offset",
                "_shortbuf",
                "_lock",
                "_offset",
                "_codecvt",
                "_wide_data",
                "_freeres_list",
                "_freeres_buf",
                "__pad5",
                "_mode",
                "_unused2",
            ];
            let values: &[&dyn::core::fmt::Debug] = &[
                &&self._flags,
                &&self._IO_read_ptr,
                &&self._IO_read_end,
                &&self._IO_read_base,
                &&self._IO_write_base,
                &&self._IO_write_ptr,
                &&self._IO_write_end,
                &&self._IO_buf_base,
                &&self._IO_buf_end,
                &&self._IO_save_base,
                &&self._IO_backup_base,
                &&self._IO_save_end,
                &&self._markers,
                &&self._chain,
                &&self._fileno,
                &&self._flags2,
                &&self._old_offset,
                &&self._cur_column,
                &&self._vtable_offset,
                &&self._shortbuf,
                &&self._lock,
                &&self._offset,
                &&self._codecvt,
                &&self._wide_data,
                &&self._freeres_list,
                &&self._freeres_buf,
                &&self.__pad5,
                &&self._mode,
                &&self._unused2,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "_IO_FILE", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_FILE {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_FILE {
        #[inline]
        fn clone(&self) -> _IO_FILE {
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_marker>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<__off_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_ushort>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_schar>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 1usize]>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_lock_t>;
            let _: ::core::clone::AssertParamIsClone<__off64_t>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_codecvt>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_wide_data>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_void>;
            let _: ::core::clone::AssertParamIsClone<size_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 20usize]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_test {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_test {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_test",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_test {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_test {
        #[inline]
        fn clone(&self) -> iperf_test {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_stream {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_stream {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_stream",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_stream {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_stream {
        #[inline]
        fn clone(&self) -> iperf_stream {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_time {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_time {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_time",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_time {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_time {
        #[inline]
        fn clone(&self) -> iperf_time {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type iperf_size_t = u64;
    extern "C" {
        pub fn iperf_get_verbose(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_control_socket(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_omit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_duration(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_role(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_reverse(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_blksize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_rate(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_pacing_timer(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_bytes(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_blocks(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_burst(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_socket_bufsize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_reporter_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_stats_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_num_streams(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_repeating_payload(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamps(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamp_format(
            ipt: *mut iperf_test,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_server_port(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_server_hostname(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_template(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_protocol_id(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output_string(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_zerocopy(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_get_server_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_unit_format(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_bind_address(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_udp_counters_64bit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_one_off(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_tos(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_extra_data(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_iperf_version() -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_no_delay(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_connect_timeout(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_dont_fragment(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_congestion_control(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_set_verbose(ipt: *mut iperf_test, verbose: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_control_socket(ipt: *mut iperf_test, ctrl_sck: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_omit(ipt: *mut iperf_test, omit: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_duration(ipt: *mut iperf_test, duration: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_reporter_interval(ipt: *mut iperf_test, reporter_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_stats_interval(ipt: *mut iperf_test, stats_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_state(ipt: *mut iperf_test, state: ::std::os::raw::c_schar);
    }
    extern "C" {
        pub fn iperf_set_test_blksize(ipt: *mut iperf_test, blksize: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_logfile(ipt: *mut iperf_test, logfile: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_rate(ipt: *mut iperf_test, rate: u64);
    }
    extern "C" {
        pub fn iperf_set_test_pacing_timer(
            ipt: *mut iperf_test,
            pacing_timer: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bytes(ipt: *mut iperf_test, bytes: u64);
    }
    extern "C" {
        pub fn iperf_set_test_blocks(ipt: *mut iperf_test, blocks: u64);
    }
    extern "C" {
        pub fn iperf_set_test_burst(ipt: *mut iperf_test, burst: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_server_port(ipt: *mut iperf_test, server_port: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_socket_bufsize(
            ipt: *mut iperf_test,
            socket_bufsize: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_num_streams(ipt: *mut iperf_test, num_streams: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_repeating_payload(
            ipt: *mut iperf_test,
            repeating_payload: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_timestamps(ipt: *mut iperf_test, timestamps: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_timestamp_format(
            arg1: *mut iperf_test,
            tf: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_role(ipt: *mut iperf_test, role: ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_server_hostname(
            ipt: *mut iperf_test,
            server_hostname: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_template(
            ipt: *mut iperf_test,
            tmp_template: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_reverse(ipt: *mut iperf_test, reverse: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_json_output(ipt: *mut iperf_test, json_output: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_has_zerocopy() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_set_test_zerocopy(ipt: *mut iperf_test, zerocopy: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_get_server_output(
            ipt: *mut iperf_test,
            get_server_output: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_unit_format(
            ipt: *mut iperf_test,
            unit_format: ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bind_address(
            ipt: *mut iperf_test,
            bind_address: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_udp_counters_64bit(
            ipt: *mut iperf_test,
            udp_counters_64bit: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_one_off(ipt: *mut iperf_test, one_off: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_tos(ipt: *mut iperf_test, tos: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_extra_data(ipt: *mut iperf_test, dat: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_bidirectional(
            ipt: *mut iperf_test,
            bidirectional: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_no_delay(ipt: *mut iperf_test, no_delay: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_dont_fragment(ipt: *mut iperf_test, dont_fragment: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_congestion_control(
            ipt: *mut iperf_test,
            cc: *mut ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_connect_timeout(ipt: *mut iperf_test, ct: ::std::os::raw::c_int);
    }
    extern "C" {
        /// exchange_parameters - handles the param_Exchange part for client
        ///
        pub fn iperf_exchange_parameters(test: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_stats_callback -- handles the statistic gathering
        ///
        pub fn iperf_stats_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_reporter_callback -- handles the report printing
        ///
        pub fn iperf_reporter_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_test -- return a new iperf_test with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_test() -> *mut iperf_test;
    }
    extern "C" {
        pub fn iperf_defaults(testp: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_test -- free resources used by test, calls iperf_free_stream to
        /// free streams
        ///
        pub fn iperf_free_test(testp: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_stream -- return a net iperf_stream with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_stream(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> *mut iperf_stream;
    }
    extern "C" {
        /// iperf_add_stream -- add a stream to a test
        ///
        pub fn iperf_add_stream(test: *mut iperf_test, stream: *mut iperf_stream);
    }
    extern "C" {
        /// iperf_init_stream -- init resources associated with test
        ///
        pub fn iperf_init_stream(
            arg1: *mut iperf_stream,
            arg2: *mut iperf_test,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_stream -- free resources associated with test
        ///
        pub fn iperf_free_stream(sp: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_set_send_state(
            test: *mut iperf_test,
            state: ::std::os::raw::c_schar,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_throttle(sp: *mut iperf_stream, nowP: *mut iperf_time);
    }
    extern "C" {
        pub fn iperf_catch_sigend(
            handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        );
    }
    extern "C" {
        pub fn iperf_got_sigend(test: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_exchange_results(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_init_test(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_send_timers(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_parse_arguments(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_open_logfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_reset_test(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_reset_stats(test: *mut iperf_test);
    }
    #[repr(C)]
    pub struct protocol {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for protocol {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "protocol",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for protocol {}
    #[automatically_derived]
    impl ::core::clone::Clone for protocol {
        #[inline]
        fn clone(&self) -> protocol {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    extern "C" {
        pub fn get_protocol(arg1: *mut iperf_test, arg2: ::std::os::raw::c_int) -> *mut protocol;
    }
    extern "C" {
        pub fn set_protocol(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_on_new_stream(arg1: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_on_test_start(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_connect(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_test_finish(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_run_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_connect(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_streams(
            arg1: *mut iperf_test,
            sender: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_client_end(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_run_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_server_listen(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_accept(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_delete_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_total_rate(arg1: *mut iperf_test, arg2: iperf_size_t);
    }
    extern "C" {
        pub fn iperf_json_start(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_json_finish(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_setaffinity(
            arg1: *mut iperf_test,
            affinity: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_clearaffinity(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_printf(
            test: *mut iperf_test,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_err(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_errexit(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_strerror(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
}
mod iperf_bindings {
    pub type size_t = ::std::os::raw::c_ulong;
    pub type __uint64_t = ::std::os::raw::c_ulong;
    pub type __off_t = ::std::os::raw::c_long;
    pub type __off64_t = ::std::os::raw::c_long;
    pub type __fd_mask = ::std::os::raw::c_long;
    #[repr(C)]
    pub struct _IO_marker {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_marker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_marker",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_marker {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_marker {
        #[inline]
        fn clone(&self) -> _IO_marker {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_codecvt {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_codecvt {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_codecvt",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_codecvt {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_codecvt {
        #[inline]
        fn clone(&self) -> _IO_codecvt {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_wide_data {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_wide_data {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_wide_data",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_wide_data {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_wide_data {
        #[inline]
        fn clone(&self) -> _IO_wide_data {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type _IO_lock_t = ::std::os::raw::c_void;
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::std::os::raw::c_int,
        pub _IO_read_ptr: *mut ::std::os::raw::c_char,
        pub _IO_read_end: *mut ::std::os::raw::c_char,
        pub _IO_read_base: *mut ::std::os::raw::c_char,
        pub _IO_write_base: *mut ::std::os::raw::c_char,
        pub _IO_write_ptr: *mut ::std::os::raw::c_char,
        pub _IO_write_end: *mut ::std::os::raw::c_char,
        pub _IO_buf_base: *mut ::std::os::raw::c_char,
        pub _IO_buf_end: *mut ::std::os::raw::c_char,
        pub _IO_save_base: *mut ::std::os::raw::c_char,
        pub _IO_backup_base: *mut ::std::os::raw::c_char,
        pub _IO_save_end: *mut ::std::os::raw::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::std::os::raw::c_int,
        pub _flags2: ::std::os::raw::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: ::std::os::raw::c_ushort,
        pub _vtable_offset: ::std::os::raw::c_schar,
        pub _shortbuf: [::std::os::raw::c_char; 1usize],
        pub _lock: *mut _IO_lock_t,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::std::os::raw::c_void,
        pub __pad5: size_t,
        pub _mode: ::std::os::raw::c_int,
        pub _unused2: [::std::os::raw::c_char; 20usize],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_FILE {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "_flags",
                "_IO_read_ptr",
                "_IO_read_end",
                "_IO_read_base",
                "_IO_write_base",
                "_IO_write_ptr",
                "_IO_write_end",
                "_IO_buf_base",
                "_IO_buf_end",
                "_IO_save_base",
                "_IO_backup_base",
                "_IO_save_end",
                "_markers",
                "_chain",
                "_fileno",
                "_flags2",
                "_old_offset",
                "_cur_column",
                "_vtable_offset",
                "_shortbuf",
                "_lock",
                "_offset",
                "_codecvt",
                "_wide_data",
                "_freeres_list",
                "_freeres_buf",
                "__pad5",
                "_mode",
                "_unused2",
            ];
            let values: &[&dyn::core::fmt::Debug] = &[
                &&self._flags,
                &&self._IO_read_ptr,
                &&self._IO_read_end,
                &&self._IO_read_base,
                &&self._IO_write_base,
                &&self._IO_write_ptr,
                &&self._IO_write_end,
                &&self._IO_buf_base,
                &&self._IO_buf_end,
                &&self._IO_save_base,
                &&self._IO_backup_base,
                &&self._IO_save_end,
                &&self._markers,
                &&self._chain,
                &&self._fileno,
                &&self._flags2,
                &&self._old_offset,
                &&self._cur_column,
                &&self._vtable_offset,
                &&self._shortbuf,
                &&self._lock,
                &&self._offset,
                &&self._codecvt,
                &&self._wide_data,
                &&self._freeres_list,
                &&self._freeres_buf,
                &&self.__pad5,
                &&self._mode,
                &&self._unused2,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "_IO_FILE", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_FILE {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_FILE {
        #[inline]
        fn clone(&self) -> _IO_FILE {
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_marker>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<__off_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_ushort>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_schar>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 1usize]>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_lock_t>;
            let _: ::core::clone::AssertParamIsClone<__off64_t>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_codecvt>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_wide_data>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_void>;
            let _: ::core::clone::AssertParamIsClone<size_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 20usize]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_test {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_test {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_test",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_test {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_test {
        #[inline]
        fn clone(&self) -> iperf_test {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_stream {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_stream {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_stream",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_stream {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_stream {
        #[inline]
        fn clone(&self) -> iperf_stream {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_time {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_time {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_time",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_time {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_time {
        #[inline]
        fn clone(&self) -> iperf_time {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type iperf_size_t = u64;
    extern "C" {
        pub fn iperf_get_verbose(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_control_socket(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_omit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_duration(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_role(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_reverse(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_blksize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_rate(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_pacing_timer(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_bytes(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_blocks(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_burst(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_socket_bufsize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_reporter_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_stats_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_num_streams(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_repeating_payload(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamps(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamp_format(
            ipt: *mut iperf_test,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_server_port(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_server_hostname(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_template(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_protocol_id(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output_string(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_zerocopy(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_get_server_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_unit_format(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_bind_address(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_udp_counters_64bit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_one_off(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_tos(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_extra_data(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_iperf_version() -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_no_delay(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_connect_timeout(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_dont_fragment(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_congestion_control(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_set_verbose(ipt: *mut iperf_test, verbose: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_control_socket(ipt: *mut iperf_test, ctrl_sck: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_omit(ipt: *mut iperf_test, omit: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_duration(ipt: *mut iperf_test, duration: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_reporter_interval(ipt: *mut iperf_test, reporter_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_stats_interval(ipt: *mut iperf_test, stats_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_state(ipt: *mut iperf_test, state: ::std::os::raw::c_schar);
    }
    extern "C" {
        pub fn iperf_set_test_blksize(ipt: *mut iperf_test, blksize: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_logfile(ipt: *mut iperf_test, logfile: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_rate(ipt: *mut iperf_test, rate: u64);
    }
    extern "C" {
        pub fn iperf_set_test_pacing_timer(
            ipt: *mut iperf_test,
            pacing_timer: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bytes(ipt: *mut iperf_test, bytes: u64);
    }
    extern "C" {
        pub fn iperf_set_test_blocks(ipt: *mut iperf_test, blocks: u64);
    }
    extern "C" {
        pub fn iperf_set_test_burst(ipt: *mut iperf_test, burst: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_server_port(ipt: *mut iperf_test, server_port: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_socket_bufsize(
            ipt: *mut iperf_test,
            socket_bufsize: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_num_streams(ipt: *mut iperf_test, num_streams: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_repeating_payload(
            ipt: *mut iperf_test,
            repeating_payload: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_timestamps(ipt: *mut iperf_test, timestamps: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_timestamp_format(
            arg1: *mut iperf_test,
            tf: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_role(ipt: *mut iperf_test, role: ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_server_hostname(
            ipt: *mut iperf_test,
            server_hostname: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_template(
            ipt: *mut iperf_test,
            tmp_template: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_reverse(ipt: *mut iperf_test, reverse: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_json_output(ipt: *mut iperf_test, json_output: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_has_zerocopy() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_set_test_zerocopy(ipt: *mut iperf_test, zerocopy: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_get_server_output(
            ipt: *mut iperf_test,
            get_server_output: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_unit_format(
            ipt: *mut iperf_test,
            unit_format: ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bind_address(
            ipt: *mut iperf_test,
            bind_address: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_udp_counters_64bit(
            ipt: *mut iperf_test,
            udp_counters_64bit: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_one_off(ipt: *mut iperf_test, one_off: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_tos(ipt: *mut iperf_test, tos: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_extra_data(ipt: *mut iperf_test, dat: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_bidirectional(
            ipt: *mut iperf_test,
            bidirectional: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_no_delay(ipt: *mut iperf_test, no_delay: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_dont_fragment(ipt: *mut iperf_test, dont_fragment: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_congestion_control(
            ipt: *mut iperf_test,
            cc: *mut ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_connect_timeout(ipt: *mut iperf_test, ct: ::std::os::raw::c_int);
    }
    extern "C" {
        /// exchange_parameters - handles the param_Exchange part for client
        ///
        pub fn iperf_exchange_parameters(test: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_stats_callback -- handles the statistic gathering
        ///
        pub fn iperf_stats_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_reporter_callback -- handles the report printing
        ///
        pub fn iperf_reporter_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_test -- return a new iperf_test with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_test() -> *mut iperf_test;
    }
    extern "C" {
        pub fn iperf_defaults(testp: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_test -- free resources used by test, calls iperf_free_stream to
        /// free streams
        ///
        pub fn iperf_free_test(testp: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_stream -- return a net iperf_stream with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_stream(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> *mut iperf_stream;
    }
    extern "C" {
        /// iperf_add_stream -- add a stream to a test
        ///
        pub fn iperf_add_stream(test: *mut iperf_test, stream: *mut iperf_stream);
    }
    extern "C" {
        /// iperf_init_stream -- init resources associated with test
        ///
        pub fn iperf_init_stream(
            arg1: *mut iperf_stream,
            arg2: *mut iperf_test,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_stream -- free resources associated with test
        ///
        pub fn iperf_free_stream(sp: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_set_send_state(
            test: *mut iperf_test,
            state: ::std::os::raw::c_schar,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_throttle(sp: *mut iperf_stream, nowP: *mut iperf_time);
    }
    extern "C" {
        pub fn iperf_catch_sigend(
            handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        );
    }
    extern "C" {
        pub fn iperf_got_sigend(test: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_exchange_results(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_init_test(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_send_timers(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_parse_arguments(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_open_logfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_reset_test(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_reset_stats(test: *mut iperf_test);
    }
    #[repr(C)]
    pub struct protocol {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for protocol {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "protocol",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for protocol {}
    #[automatically_derived]
    impl ::core::clone::Clone for protocol {
        #[inline]
        fn clone(&self) -> protocol {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    extern "C" {
        pub fn get_protocol(arg1: *mut iperf_test, arg2: ::std::os::raw::c_int) -> *mut protocol;
    }
    extern "C" {
        pub fn set_protocol(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_on_new_stream(arg1: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_on_test_start(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_connect(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_test_finish(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_run_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_connect(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_streams(
            arg1: *mut iperf_test,
            sender: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_client_end(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_run_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_server_listen(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_accept(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_delete_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_total_rate(arg1: *mut iperf_test, arg2: iperf_size_t);
    }
    extern "C" {
        pub fn iperf_json_start(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_json_finish(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_setaffinity(
            arg1: *mut iperf_test,
            affinity: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_clearaffinity(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_printf(
            test: *mut iperf_test,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_err(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_errexit(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_strerror(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
}
mod iperf_bindings {
    #![allow(non_camel_case_types)]
    pub type size_t = ::std::os::raw::c_ulong;
    pub type __uint64_t = ::std::os::raw::c_ulong;
    pub type __off_t = ::std::os::raw::c_long;
    pub type __off64_t = ::std::os::raw::c_long;
    pub type __fd_mask = ::std::os::raw::c_long;
    #[repr(C)]
    pub struct _IO_marker {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_marker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_marker",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_marker {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_marker {
        #[inline]
        fn clone(&self) -> _IO_marker {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_codecvt {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_codecvt {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_codecvt",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_codecvt {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_codecvt {
        #[inline]
        fn clone(&self) -> _IO_codecvt {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_wide_data {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_wide_data {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_wide_data",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_wide_data {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_wide_data {
        #[inline]
        fn clone(&self) -> _IO_wide_data {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type _IO_lock_t = ::std::os::raw::c_void;
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::std::os::raw::c_int,
        pub _IO_read_ptr: *mut ::std::os::raw::c_char,
        pub _IO_read_end: *mut ::std::os::raw::c_char,
        pub _IO_read_base: *mut ::std::os::raw::c_char,
        pub _IO_write_base: *mut ::std::os::raw::c_char,
        pub _IO_write_ptr: *mut ::std::os::raw::c_char,
        pub _IO_write_end: *mut ::std::os::raw::c_char,
        pub _IO_buf_base: *mut ::std::os::raw::c_char,
        pub _IO_buf_end: *mut ::std::os::raw::c_char,
        pub _IO_save_base: *mut ::std::os::raw::c_char,
        pub _IO_backup_base: *mut ::std::os::raw::c_char,
        pub _IO_save_end: *mut ::std::os::raw::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::std::os::raw::c_int,
        pub _flags2: ::std::os::raw::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: ::std::os::raw::c_ushort,
        pub _vtable_offset: ::std::os::raw::c_schar,
        pub _shortbuf: [::std::os::raw::c_char; 1usize],
        pub _lock: *mut _IO_lock_t,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::std::os::raw::c_void,
        pub __pad5: size_t,
        pub _mode: ::std::os::raw::c_int,
        pub _unused2: [::std::os::raw::c_char; 20usize],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_FILE {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "_flags",
                "_IO_read_ptr",
                "_IO_read_end",
                "_IO_read_base",
                "_IO_write_base",
                "_IO_write_ptr",
                "_IO_write_end",
                "_IO_buf_base",
                "_IO_buf_end",
                "_IO_save_base",
                "_IO_backup_base",
                "_IO_save_end",
                "_markers",
                "_chain",
                "_fileno",
                "_flags2",
                "_old_offset",
                "_cur_column",
                "_vtable_offset",
                "_shortbuf",
                "_lock",
                "_offset",
                "_codecvt",
                "_wide_data",
                "_freeres_list",
                "_freeres_buf",
                "__pad5",
                "_mode",
                "_unused2",
            ];
            let values: &[&dyn::core::fmt::Debug] = &[
                &&self._flags,
                &&self._IO_read_ptr,
                &&self._IO_read_end,
                &&self._IO_read_base,
                &&self._IO_write_base,
                &&self._IO_write_ptr,
                &&self._IO_write_end,
                &&self._IO_buf_base,
                &&self._IO_buf_end,
                &&self._IO_save_base,
                &&self._IO_backup_base,
                &&self._IO_save_end,
                &&self._markers,
                &&self._chain,
                &&self._fileno,
                &&self._flags2,
                &&self._old_offset,
                &&self._cur_column,
                &&self._vtable_offset,
                &&self._shortbuf,
                &&self._lock,
                &&self._offset,
                &&self._codecvt,
                &&self._wide_data,
                &&self._freeres_list,
                &&self._freeres_buf,
                &&self.__pad5,
                &&self._mode,
                &&self._unused2,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "_IO_FILE", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_FILE {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_FILE {
        #[inline]
        fn clone(&self) -> _IO_FILE {
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_marker>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<__off_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_ushort>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_schar>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 1usize]>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_lock_t>;
            let _: ::core::clone::AssertParamIsClone<__off64_t>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_codecvt>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_wide_data>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_void>;
            let _: ::core::clone::AssertParamIsClone<size_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 20usize]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_test {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_test {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_test",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_test {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_test {
        #[inline]
        fn clone(&self) -> iperf_test {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_stream {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_stream {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_stream",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_stream {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_stream {
        #[inline]
        fn clone(&self) -> iperf_stream {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_time {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_time {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_time",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_time {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_time {
        #[inline]
        fn clone(&self) -> iperf_time {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type iperf_size_t = u64;
    extern "C" {
        pub fn iperf_get_verbose(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_control_socket(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_omit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_duration(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_role(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_reverse(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_blksize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_rate(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_pacing_timer(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_bytes(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_blocks(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_burst(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_socket_bufsize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_reporter_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_stats_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_num_streams(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_repeating_payload(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamps(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamp_format(
            ipt: *mut iperf_test,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_server_port(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_server_hostname(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_template(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_protocol_id(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output_string(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_zerocopy(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_get_server_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_unit_format(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_bind_address(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_udp_counters_64bit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_one_off(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_tos(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_extra_data(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_iperf_version() -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_no_delay(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_connect_timeout(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_dont_fragment(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_congestion_control(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_set_verbose(ipt: *mut iperf_test, verbose: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_control_socket(ipt: *mut iperf_test, ctrl_sck: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_omit(ipt: *mut iperf_test, omit: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_duration(ipt: *mut iperf_test, duration: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_reporter_interval(ipt: *mut iperf_test, reporter_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_stats_interval(ipt: *mut iperf_test, stats_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_state(ipt: *mut iperf_test, state: ::std::os::raw::c_schar);
    }
    extern "C" {
        pub fn iperf_set_test_blksize(ipt: *mut iperf_test, blksize: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_logfile(ipt: *mut iperf_test, logfile: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_rate(ipt: *mut iperf_test, rate: u64);
    }
    extern "C" {
        pub fn iperf_set_test_pacing_timer(
            ipt: *mut iperf_test,
            pacing_timer: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bytes(ipt: *mut iperf_test, bytes: u64);
    }
    extern "C" {
        pub fn iperf_set_test_blocks(ipt: *mut iperf_test, blocks: u64);
    }
    extern "C" {
        pub fn iperf_set_test_burst(ipt: *mut iperf_test, burst: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_server_port(ipt: *mut iperf_test, server_port: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_socket_bufsize(
            ipt: *mut iperf_test,
            socket_bufsize: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_num_streams(ipt: *mut iperf_test, num_streams: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_repeating_payload(
            ipt: *mut iperf_test,
            repeating_payload: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_timestamps(ipt: *mut iperf_test, timestamps: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_timestamp_format(
            arg1: *mut iperf_test,
            tf: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_role(ipt: *mut iperf_test, role: ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_server_hostname(
            ipt: *mut iperf_test,
            server_hostname: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_template(
            ipt: *mut iperf_test,
            tmp_template: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_reverse(ipt: *mut iperf_test, reverse: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_json_output(ipt: *mut iperf_test, json_output: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_has_zerocopy() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_set_test_zerocopy(ipt: *mut iperf_test, zerocopy: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_get_server_output(
            ipt: *mut iperf_test,
            get_server_output: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_unit_format(
            ipt: *mut iperf_test,
            unit_format: ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bind_address(
            ipt: *mut iperf_test,
            bind_address: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_udp_counters_64bit(
            ipt: *mut iperf_test,
            udp_counters_64bit: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_one_off(ipt: *mut iperf_test, one_off: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_tos(ipt: *mut iperf_test, tos: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_extra_data(ipt: *mut iperf_test, dat: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_bidirectional(
            ipt: *mut iperf_test,
            bidirectional: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_no_delay(ipt: *mut iperf_test, no_delay: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_dont_fragment(ipt: *mut iperf_test, dont_fragment: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_congestion_control(
            ipt: *mut iperf_test,
            cc: *mut ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_connect_timeout(ipt: *mut iperf_test, ct: ::std::os::raw::c_int);
    }
    extern "C" {
        /// exchange_parameters - handles the param_Exchange part for client
        ///
        pub fn iperf_exchange_parameters(test: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_stats_callback -- handles the statistic gathering
        ///
        pub fn iperf_stats_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_reporter_callback -- handles the report printing
        ///
        pub fn iperf_reporter_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_test -- return a new iperf_test with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_test() -> *mut iperf_test;
    }
    extern "C" {
        pub fn iperf_defaults(testp: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_test -- free resources used by test, calls iperf_free_stream to
        /// free streams
        ///
        pub fn iperf_free_test(testp: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_stream -- return a net iperf_stream with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_stream(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> *mut iperf_stream;
    }
    extern "C" {
        /// iperf_add_stream -- add a stream to a test
        ///
        pub fn iperf_add_stream(test: *mut iperf_test, stream: *mut iperf_stream);
    }
    extern "C" {
        /// iperf_init_stream -- init resources associated with test
        ///
        pub fn iperf_init_stream(
            arg1: *mut iperf_stream,
            arg2: *mut iperf_test,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_stream -- free resources associated with test
        ///
        pub fn iperf_free_stream(sp: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_set_send_state(
            test: *mut iperf_test,
            state: ::std::os::raw::c_schar,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_throttle(sp: *mut iperf_stream, nowP: *mut iperf_time);
    }
    extern "C" {
        pub fn iperf_catch_sigend(
            handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        );
    }
    extern "C" {
        pub fn iperf_got_sigend(test: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_exchange_results(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_init_test(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_send_timers(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_parse_arguments(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_open_logfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_reset_test(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_reset_stats(test: *mut iperf_test);
    }
    #[repr(C)]
    pub struct protocol {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for protocol {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "protocol",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for protocol {}
    #[automatically_derived]
    impl ::core::clone::Clone for protocol {
        #[inline]
        fn clone(&self) -> protocol {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    extern "C" {
        pub fn get_protocol(arg1: *mut iperf_test, arg2: ::std::os::raw::c_int) -> *mut protocol;
    }
    extern "C" {
        pub fn set_protocol(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_on_new_stream(arg1: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_on_test_start(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_connect(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_test_finish(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_run_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_connect(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_streams(
            arg1: *mut iperf_test,
            sender: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_client_end(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_run_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_server_listen(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_accept(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_delete_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_total_rate(arg1: *mut iperf_test, arg2: iperf_size_t);
    }
    extern "C" {
        pub fn iperf_json_start(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_json_finish(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_setaffinity(
            arg1: *mut iperf_test,
            affinity: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_clearaffinity(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_printf(
            test: *mut iperf_test,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_err(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_errexit(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_strerror(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
}
mod iperf_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    pub type size_t = ::std::os::raw::c_ulong;
    pub type __uint64_t = ::std::os::raw::c_ulong;
    pub type __off_t = ::std::os::raw::c_long;
    pub type __off64_t = ::std::os::raw::c_long;
    pub type __fd_mask = ::std::os::raw::c_long;
    #[repr(C)]
    pub struct _IO_marker {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_marker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_marker",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_marker {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_marker {
        #[inline]
        fn clone(&self) -> _IO_marker {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_codecvt {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_codecvt {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_codecvt",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_codecvt {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_codecvt {
        #[inline]
        fn clone(&self) -> _IO_codecvt {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_wide_data {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_wide_data {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_wide_data",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_wide_data {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_wide_data {
        #[inline]
        fn clone(&self) -> _IO_wide_data {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type _IO_lock_t = ::std::os::raw::c_void;
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::std::os::raw::c_int,
        pub _IO_read_ptr: *mut ::std::os::raw::c_char,
        pub _IO_read_end: *mut ::std::os::raw::c_char,
        pub _IO_read_base: *mut ::std::os::raw::c_char,
        pub _IO_write_base: *mut ::std::os::raw::c_char,
        pub _IO_write_ptr: *mut ::std::os::raw::c_char,
        pub _IO_write_end: *mut ::std::os::raw::c_char,
        pub _IO_buf_base: *mut ::std::os::raw::c_char,
        pub _IO_buf_end: *mut ::std::os::raw::c_char,
        pub _IO_save_base: *mut ::std::os::raw::c_char,
        pub _IO_backup_base: *mut ::std::os::raw::c_char,
        pub _IO_save_end: *mut ::std::os::raw::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::std::os::raw::c_int,
        pub _flags2: ::std::os::raw::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: ::std::os::raw::c_ushort,
        pub _vtable_offset: ::std::os::raw::c_schar,
        pub _shortbuf: [::std::os::raw::c_char; 1usize],
        pub _lock: *mut _IO_lock_t,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::std::os::raw::c_void,
        pub __pad5: size_t,
        pub _mode: ::std::os::raw::c_int,
        pub _unused2: [::std::os::raw::c_char; 20usize],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_FILE {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "_flags",
                "_IO_read_ptr",
                "_IO_read_end",
                "_IO_read_base",
                "_IO_write_base",
                "_IO_write_ptr",
                "_IO_write_end",
                "_IO_buf_base",
                "_IO_buf_end",
                "_IO_save_base",
                "_IO_backup_base",
                "_IO_save_end",
                "_markers",
                "_chain",
                "_fileno",
                "_flags2",
                "_old_offset",
                "_cur_column",
                "_vtable_offset",
                "_shortbuf",
                "_lock",
                "_offset",
                "_codecvt",
                "_wide_data",
                "_freeres_list",
                "_freeres_buf",
                "__pad5",
                "_mode",
                "_unused2",
            ];
            let values: &[&dyn::core::fmt::Debug] = &[
                &&self._flags,
                &&self._IO_read_ptr,
                &&self._IO_read_end,
                &&self._IO_read_base,
                &&self._IO_write_base,
                &&self._IO_write_ptr,
                &&self._IO_write_end,
                &&self._IO_buf_base,
                &&self._IO_buf_end,
                &&self._IO_save_base,
                &&self._IO_backup_base,
                &&self._IO_save_end,
                &&self._markers,
                &&self._chain,
                &&self._fileno,
                &&self._flags2,
                &&self._old_offset,
                &&self._cur_column,
                &&self._vtable_offset,
                &&self._shortbuf,
                &&self._lock,
                &&self._offset,
                &&self._codecvt,
                &&self._wide_data,
                &&self._freeres_list,
                &&self._freeres_buf,
                &&self.__pad5,
                &&self._mode,
                &&self._unused2,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "_IO_FILE", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_FILE {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_FILE {
        #[inline]
        fn clone(&self) -> _IO_FILE {
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_marker>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<__off_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_ushort>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_schar>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 1usize]>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_lock_t>;
            let _: ::core::clone::AssertParamIsClone<__off64_t>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_codecvt>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_wide_data>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_void>;
            let _: ::core::clone::AssertParamIsClone<size_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 20usize]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_test {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_test {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_test",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_test {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_test {
        #[inline]
        fn clone(&self) -> iperf_test {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_stream {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_stream {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_stream",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_stream {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_stream {
        #[inline]
        fn clone(&self) -> iperf_stream {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_time {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_time {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_time",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_time {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_time {
        #[inline]
        fn clone(&self) -> iperf_time {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type iperf_size_t = u64;
    extern "C" {
        pub fn iperf_get_verbose(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_control_socket(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_omit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_duration(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_role(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_reverse(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_blksize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_rate(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_pacing_timer(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_bytes(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_blocks(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_burst(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_socket_bufsize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_reporter_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_stats_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_num_streams(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_repeating_payload(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamps(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamp_format(
            ipt: *mut iperf_test,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_server_port(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_server_hostname(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_template(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_protocol_id(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output_string(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_zerocopy(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_get_server_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_unit_format(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_bind_address(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_udp_counters_64bit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_one_off(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_tos(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_extra_data(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_iperf_version() -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_no_delay(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_connect_timeout(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_dont_fragment(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_congestion_control(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_set_verbose(ipt: *mut iperf_test, verbose: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_control_socket(ipt: *mut iperf_test, ctrl_sck: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_omit(ipt: *mut iperf_test, omit: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_duration(ipt: *mut iperf_test, duration: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_reporter_interval(ipt: *mut iperf_test, reporter_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_stats_interval(ipt: *mut iperf_test, stats_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_state(ipt: *mut iperf_test, state: ::std::os::raw::c_schar);
    }
    extern "C" {
        pub fn iperf_set_test_blksize(ipt: *mut iperf_test, blksize: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_logfile(ipt: *mut iperf_test, logfile: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_rate(ipt: *mut iperf_test, rate: u64);
    }
    extern "C" {
        pub fn iperf_set_test_pacing_timer(
            ipt: *mut iperf_test,
            pacing_timer: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bytes(ipt: *mut iperf_test, bytes: u64);
    }
    extern "C" {
        pub fn iperf_set_test_blocks(ipt: *mut iperf_test, blocks: u64);
    }
    extern "C" {
        pub fn iperf_set_test_burst(ipt: *mut iperf_test, burst: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_server_port(ipt: *mut iperf_test, server_port: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_socket_bufsize(
            ipt: *mut iperf_test,
            socket_bufsize: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_num_streams(ipt: *mut iperf_test, num_streams: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_repeating_payload(
            ipt: *mut iperf_test,
            repeating_payload: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_timestamps(ipt: *mut iperf_test, timestamps: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_timestamp_format(
            arg1: *mut iperf_test,
            tf: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_role(ipt: *mut iperf_test, role: ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_server_hostname(
            ipt: *mut iperf_test,
            server_hostname: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_template(
            ipt: *mut iperf_test,
            tmp_template: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_reverse(ipt: *mut iperf_test, reverse: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_json_output(ipt: *mut iperf_test, json_output: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_has_zerocopy() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_set_test_zerocopy(ipt: *mut iperf_test, zerocopy: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_get_server_output(
            ipt: *mut iperf_test,
            get_server_output: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_unit_format(
            ipt: *mut iperf_test,
            unit_format: ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bind_address(
            ipt: *mut iperf_test,
            bind_address: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_udp_counters_64bit(
            ipt: *mut iperf_test,
            udp_counters_64bit: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_one_off(ipt: *mut iperf_test, one_off: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_tos(ipt: *mut iperf_test, tos: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_extra_data(ipt: *mut iperf_test, dat: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_bidirectional(
            ipt: *mut iperf_test,
            bidirectional: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_no_delay(ipt: *mut iperf_test, no_delay: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_dont_fragment(ipt: *mut iperf_test, dont_fragment: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_congestion_control(
            ipt: *mut iperf_test,
            cc: *mut ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_connect_timeout(ipt: *mut iperf_test, ct: ::std::os::raw::c_int);
    }
    extern "C" {
        /// exchange_parameters - handles the param_Exchange part for client
        ///
        pub fn iperf_exchange_parameters(test: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_stats_callback -- handles the statistic gathering
        ///
        pub fn iperf_stats_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_reporter_callback -- handles the report printing
        ///
        pub fn iperf_reporter_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_test -- return a new iperf_test with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_test() -> *mut iperf_test;
    }
    extern "C" {
        pub fn iperf_defaults(testp: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_test -- free resources used by test, calls iperf_free_stream to
        /// free streams
        ///
        pub fn iperf_free_test(testp: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_stream -- return a net iperf_stream with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_stream(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> *mut iperf_stream;
    }
    extern "C" {
        /// iperf_add_stream -- add a stream to a test
        ///
        pub fn iperf_add_stream(test: *mut iperf_test, stream: *mut iperf_stream);
    }
    extern "C" {
        /// iperf_init_stream -- init resources associated with test
        ///
        pub fn iperf_init_stream(
            arg1: *mut iperf_stream,
            arg2: *mut iperf_test,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_stream -- free resources associated with test
        ///
        pub fn iperf_free_stream(sp: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_set_send_state(
            test: *mut iperf_test,
            state: ::std::os::raw::c_schar,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_throttle(sp: *mut iperf_stream, nowP: *mut iperf_time);
    }
    extern "C" {
        pub fn iperf_catch_sigend(
            handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        );
    }
    extern "C" {
        pub fn iperf_got_sigend(test: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_exchange_results(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_init_test(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_send_timers(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_parse_arguments(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_open_logfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_reset_test(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_reset_stats(test: *mut iperf_test);
    }
    #[repr(C)]
    pub struct protocol {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for protocol {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "protocol",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for protocol {}
    #[automatically_derived]
    impl ::core::clone::Clone for protocol {
        #[inline]
        fn clone(&self) -> protocol {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    extern "C" {
        pub fn get_protocol(arg1: *mut iperf_test, arg2: ::std::os::raw::c_int) -> *mut protocol;
    }
    extern "C" {
        pub fn set_protocol(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_on_new_stream(arg1: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_on_test_start(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_connect(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_test_finish(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_run_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_connect(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_streams(
            arg1: *mut iperf_test,
            sender: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_client_end(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_run_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_server_listen(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_accept(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_delete_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_total_rate(arg1: *mut iperf_test, arg2: iperf_size_t);
    }
    extern "C" {
        pub fn iperf_json_start(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_json_finish(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_setaffinity(
            arg1: *mut iperf_test,
            affinity: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_clearaffinity(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_printf(
            test: *mut iperf_test,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_err(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_errexit(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_strerror(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
}
mod iperf_bindings {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    pub type size_t = ::std::os::raw::c_ulong;
    pub type __uint64_t = ::std::os::raw::c_ulong;
    pub type __off_t = ::std::os::raw::c_long;
    pub type __off64_t = ::std::os::raw::c_long;
    pub type __fd_mask = ::std::os::raw::c_long;
    #[repr(C)]
    pub struct _IO_marker {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_marker {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_marker",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_marker {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_marker {
        #[inline]
        fn clone(&self) -> _IO_marker {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_codecvt {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_codecvt {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_codecvt",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_codecvt {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_codecvt {
        #[inline]
        fn clone(&self) -> _IO_codecvt {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct _IO_wide_data {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_wide_data {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "_IO_wide_data",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_wide_data {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_wide_data {
        #[inline]
        fn clone(&self) -> _IO_wide_data {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type _IO_lock_t = ::std::os::raw::c_void;
    #[repr(C)]
    pub struct _IO_FILE {
        pub _flags: ::std::os::raw::c_int,
        pub _IO_read_ptr: *mut ::std::os::raw::c_char,
        pub _IO_read_end: *mut ::std::os::raw::c_char,
        pub _IO_read_base: *mut ::std::os::raw::c_char,
        pub _IO_write_base: *mut ::std::os::raw::c_char,
        pub _IO_write_ptr: *mut ::std::os::raw::c_char,
        pub _IO_write_end: *mut ::std::os::raw::c_char,
        pub _IO_buf_base: *mut ::std::os::raw::c_char,
        pub _IO_buf_end: *mut ::std::os::raw::c_char,
        pub _IO_save_base: *mut ::std::os::raw::c_char,
        pub _IO_backup_base: *mut ::std::os::raw::c_char,
        pub _IO_save_end: *mut ::std::os::raw::c_char,
        pub _markers: *mut _IO_marker,
        pub _chain: *mut _IO_FILE,
        pub _fileno: ::std::os::raw::c_int,
        pub _flags2: ::std::os::raw::c_int,
        pub _old_offset: __off_t,
        pub _cur_column: ::std::os::raw::c_ushort,
        pub _vtable_offset: ::std::os::raw::c_schar,
        pub _shortbuf: [::std::os::raw::c_char; 1usize],
        pub _lock: *mut _IO_lock_t,
        pub _offset: __off64_t,
        pub _codecvt: *mut _IO_codecvt,
        pub _wide_data: *mut _IO_wide_data,
        pub _freeres_list: *mut _IO_FILE,
        pub _freeres_buf: *mut ::std::os::raw::c_void,
        pub __pad5: size_t,
        pub _mode: ::std::os::raw::c_int,
        pub _unused2: [::std::os::raw::c_char; 20usize],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for _IO_FILE {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "_flags",
                "_IO_read_ptr",
                "_IO_read_end",
                "_IO_read_base",
                "_IO_write_base",
                "_IO_write_ptr",
                "_IO_write_end",
                "_IO_buf_base",
                "_IO_buf_end",
                "_IO_save_base",
                "_IO_backup_base",
                "_IO_save_end",
                "_markers",
                "_chain",
                "_fileno",
                "_flags2",
                "_old_offset",
                "_cur_column",
                "_vtable_offset",
                "_shortbuf",
                "_lock",
                "_offset",
                "_codecvt",
                "_wide_data",
                "_freeres_list",
                "_freeres_buf",
                "__pad5",
                "_mode",
                "_unused2",
            ];
            let values: &[&dyn::core::fmt::Debug] = &[
                &&self._flags,
                &&self._IO_read_ptr,
                &&self._IO_read_end,
                &&self._IO_read_base,
                &&self._IO_write_base,
                &&self._IO_write_ptr,
                &&self._IO_write_end,
                &&self._IO_buf_base,
                &&self._IO_buf_end,
                &&self._IO_save_base,
                &&self._IO_backup_base,
                &&self._IO_save_end,
                &&self._markers,
                &&self._chain,
                &&self._fileno,
                &&self._flags2,
                &&self._old_offset,
                &&self._cur_column,
                &&self._vtable_offset,
                &&self._shortbuf,
                &&self._lock,
                &&self._offset,
                &&self._codecvt,
                &&self._wide_data,
                &&self._freeres_list,
                &&self._freeres_buf,
                &&self.__pad5,
                &&self._mode,
                &&self._unused2,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(f, "_IO_FILE", names, values)
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for _IO_FILE {}
    #[automatically_derived]
    impl ::core::clone::Clone for _IO_FILE {
        #[inline]
        fn clone(&self) -> _IO_FILE {
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_char>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_marker>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<__off_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_ushort>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_schar>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 1usize]>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_lock_t>;
            let _: ::core::clone::AssertParamIsClone<__off64_t>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_codecvt>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_wide_data>;
            let _: ::core::clone::AssertParamIsClone<*mut _IO_FILE>;
            let _: ::core::clone::AssertParamIsClone<*mut ::std::os::raw::c_void>;
            let _: ::core::clone::AssertParamIsClone<size_t>;
            let _: ::core::clone::AssertParamIsClone<::std::os::raw::c_int>;
            let _: ::core::clone::AssertParamIsClone<[::std::os::raw::c_char; 20usize]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_test {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_test {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_test",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_test {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_test {
        #[inline]
        fn clone(&self) -> iperf_test {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_stream {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_stream {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_stream",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_stream {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_stream {
        #[inline]
        fn clone(&self) -> iperf_stream {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    #[repr(C)]
    pub struct iperf_time {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for iperf_time {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "iperf_time",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for iperf_time {}
    #[automatically_derived]
    impl ::core::clone::Clone for iperf_time {
        #[inline]
        fn clone(&self) -> iperf_time {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    pub type iperf_size_t = u64;
    extern "C" {
        pub fn iperf_get_verbose(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_control_socket(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_omit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_duration(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_role(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_reverse(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_blksize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_rate(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_pacing_timer(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_bytes(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_blocks(ipt: *mut iperf_test) -> u64;
    }
    extern "C" {
        pub fn iperf_get_test_burst(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_socket_bufsize(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_reporter_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_stats_interval(ipt: *mut iperf_test) -> f64;
    }
    extern "C" {
        pub fn iperf_get_test_num_streams(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_repeating_payload(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamps(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_timestamp_format(
            ipt: *mut iperf_test,
        ) -> *const ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_server_port(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_server_hostname(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_template(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_protocol_id(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_json_output_string(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_zerocopy(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_get_server_output(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_unit_format(ipt: *mut iperf_test) -> ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_bind_address(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_udp_counters_64bit(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_one_off(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_tos(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_extra_data(ipt: *mut iperf_test) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_iperf_version() -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_get_test_no_delay(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_connect_timeout(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_dont_fragment(ipt: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_get_test_congestion_control(
            ipt: *mut iperf_test,
        ) -> *mut ::std::os::raw::c_char;
    }
    extern "C" {
        pub fn iperf_set_verbose(ipt: *mut iperf_test, verbose: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_control_socket(ipt: *mut iperf_test, ctrl_sck: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_omit(ipt: *mut iperf_test, omit: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_duration(ipt: *mut iperf_test, duration: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_reporter_interval(ipt: *mut iperf_test, reporter_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_stats_interval(ipt: *mut iperf_test, stats_interval: f64);
    }
    extern "C" {
        pub fn iperf_set_test_state(ipt: *mut iperf_test, state: ::std::os::raw::c_schar);
    }
    extern "C" {
        pub fn iperf_set_test_blksize(ipt: *mut iperf_test, blksize: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_logfile(ipt: *mut iperf_test, logfile: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_rate(ipt: *mut iperf_test, rate: u64);
    }
    extern "C" {
        pub fn iperf_set_test_pacing_timer(
            ipt: *mut iperf_test,
            pacing_timer: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bytes(ipt: *mut iperf_test, bytes: u64);
    }
    extern "C" {
        pub fn iperf_set_test_blocks(ipt: *mut iperf_test, blocks: u64);
    }
    extern "C" {
        pub fn iperf_set_test_burst(ipt: *mut iperf_test, burst: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_server_port(ipt: *mut iperf_test, server_port: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_socket_bufsize(
            ipt: *mut iperf_test,
            socket_bufsize: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_num_streams(ipt: *mut iperf_test, num_streams: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_repeating_payload(
            ipt: *mut iperf_test,
            repeating_payload: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_timestamps(ipt: *mut iperf_test, timestamps: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_timestamp_format(
            arg1: *mut iperf_test,
            tf: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_role(ipt: *mut iperf_test, role: ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_server_hostname(
            ipt: *mut iperf_test,
            server_hostname: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_template(
            ipt: *mut iperf_test,
            tmp_template: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_reverse(ipt: *mut iperf_test, reverse: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_json_output(ipt: *mut iperf_test, json_output: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_has_zerocopy() -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_set_test_zerocopy(ipt: *mut iperf_test, zerocopy: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_get_server_output(
            ipt: *mut iperf_test,
            get_server_output: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_unit_format(
            ipt: *mut iperf_test,
            unit_format: ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_bind_address(
            ipt: *mut iperf_test,
            bind_address: *const ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_udp_counters_64bit(
            ipt: *mut iperf_test,
            udp_counters_64bit: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_one_off(ipt: *mut iperf_test, one_off: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_tos(ipt: *mut iperf_test, tos: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_extra_data(ipt: *mut iperf_test, dat: *const ::std::os::raw::c_char);
    }
    extern "C" {
        pub fn iperf_set_test_bidirectional(
            ipt: *mut iperf_test,
            bidirectional: ::std::os::raw::c_int,
        );
    }
    extern "C" {
        pub fn iperf_set_test_no_delay(ipt: *mut iperf_test, no_delay: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_dont_fragment(ipt: *mut iperf_test, dont_fragment: ::std::os::raw::c_int);
    }
    extern "C" {
        pub fn iperf_set_test_congestion_control(
            ipt: *mut iperf_test,
            cc: *mut ::std::os::raw::c_char,
        );
    }
    extern "C" {
        pub fn iperf_set_test_connect_timeout(ipt: *mut iperf_test, ct: ::std::os::raw::c_int);
    }
    extern "C" {
        /// exchange_parameters - handles the param_Exchange part for client
        ///
        pub fn iperf_exchange_parameters(test: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_stats_callback -- handles the statistic gathering
        ///
        pub fn iperf_stats_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_reporter_callback -- handles the report printing
        ///
        pub fn iperf_reporter_callback(test: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_test -- return a new iperf_test with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_test() -> *mut iperf_test;
    }
    extern "C" {
        pub fn iperf_defaults(testp: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_test -- free resources used by test, calls iperf_free_stream to
        /// free streams
        ///
        pub fn iperf_free_test(testp: *mut iperf_test);
    }
    extern "C" {
        /// iperf_new_stream -- return a net iperf_stream with default values
        ///
        /// returns NULL on failure
        ///
        pub fn iperf_new_stream(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: ::std::os::raw::c_int,
        ) -> *mut iperf_stream;
    }
    extern "C" {
        /// iperf_add_stream -- add a stream to a test
        ///
        pub fn iperf_add_stream(test: *mut iperf_test, stream: *mut iperf_stream);
    }
    extern "C" {
        /// iperf_init_stream -- init resources associated with test
        ///
        pub fn iperf_init_stream(
            arg1: *mut iperf_stream,
            arg2: *mut iperf_test,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        /// iperf_free_stream -- free resources associated with test
        ///
        pub fn iperf_free_stream(sp: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_set_send_state(
            test: *mut iperf_test,
            state: ::std::os::raw::c_schar,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_throttle(sp: *mut iperf_stream, nowP: *mut iperf_time);
    }
    extern "C" {
        pub fn iperf_catch_sigend(
            handler: ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        );
    }
    extern "C" {
        pub fn iperf_got_sigend(test: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_exchange_results(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_init_test(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_send_timers(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_parse_arguments(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
            arg3: *mut *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_open_logfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_reset_test(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_reset_stats(test: *mut iperf_test);
    }
    #[repr(C)]
    pub struct protocol {
        _unused: [u8; 0],
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for protocol {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "protocol",
                "_unused",
                &&self._unused,
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for protocol {}
    #[automatically_derived]
    impl ::core::clone::Clone for protocol {
        #[inline]
        fn clone(&self) -> protocol {
            let _: ::core::clone::AssertParamIsClone<[u8; 0]>;
            *self
        }
    }
    extern "C" {
        pub fn get_protocol(arg1: *mut iperf_test, arg2: ::std::os::raw::c_int) -> *mut protocol;
    }
    extern "C" {
        pub fn set_protocol(
            arg1: *mut iperf_test,
            arg2: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_on_new_stream(arg1: *mut iperf_stream);
    }
    extern "C" {
        pub fn iperf_on_test_start(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_connect(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_on_test_finish(arg1: *mut iperf_test);
    }
    extern "C" {
        pub fn iperf_run_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_connect(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_streams(
            arg1: *mut iperf_test,
            sender: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_client(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_client_end(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_run_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_server_listen(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_accept(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_handle_message_server(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_create_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_delete_pidfile(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_check_total_rate(arg1: *mut iperf_test, arg2: iperf_size_t);
    }
    extern "C" {
        pub fn iperf_json_start(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_json_finish(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_setaffinity(
            arg1: *mut iperf_test,
            affinity: ::std::os::raw::c_int,
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_clearaffinity(arg1: *mut iperf_test) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_printf(
            test: *mut iperf_test,
            format: *const ::std::os::raw::c_char,
            ...
        ) -> ::std::os::raw::c_int;
    }
    extern "C" {
        pub fn iperf_err(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_errexit(test: *mut iperf_test, format: *const ::std::os::raw::c_char, ...);
    }
    extern "C" {
        pub fn iperf_strerror(arg1: ::std::os::raw::c_int) -> *mut ::std::os::raw::c_char;
    }
}
