A sample program to demonstrate the usage of 'jemalloc' for memory leak detection.


src/main.rs - A program that leak memory.
	

1. build with profiling

```
	$ cargo build --release --features tikv-jemallocator/profiling
```

2. run with profile and leak detection

```
	$ MALLOC_CONF=prof_leak:true,lg_prof_sample:0,prof_final:true target/debug/hello
```

3. analyze with 'jeprof'

```
sol@pop-net-sv15-1:~/src/hello$ ~/src/jemalloc/bin/jeprof --show_bytes target/debug/hello jeprof.397079.0.f.heap
Using local file target/debug/hello.
Using local file jeprof.397079.0.f.heap.
Welcome to jeprof!  For help, type 'help'.
(jeprof) top
Total: 40960 B
   40960 100.0% 100.0%    40960 100.0% hello::main
       0   0.0% 100.0%    40960 100.0% __libc_start_main
       0   0.0% 100.0%    40960 100.0% _start
       0   0.0% 100.0%    40960 100.0% core::ops::function::FnOnce::call_once
       0   0.0% 100.0%    40960 100.0% core::ops::function::impls::::call_once (inline)
       0   0.0% 100.0%    40960 100.0% main
       0   0.0% 100.0%    40960 100.0% std::panic::catch_unwind (inline)
       0   0.0% 100.0%    40960 100.0% std::panicking::try (inline)
       0   0.0% 100.0%    40960 100.0% std::panicking::try::do_call (inline)
       0   0.0% 100.0%    40960 100.0% std::rt::lang_start


(jeprof) list main
Total: 40960 B
ROUTINE ====================== hello::main in /home/sol/src/hello/src/main.rs
 40960  40960 Total B (flat / cumulative)
     .      .   10: static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
     .      .   11:
     .      .   12:
     .      .   13: extern crate libc; // 0.2.65
     .      .   14:
---
     .      .   15: use std::mem;
     .      .   16:
     .      .   17:
     .      .   18: fn main()
     .      .   19: {
     .      .   20:    for _i in 0..10 {
 40960  40960   21:         //println!("allocate ...");
     .      .   22:
     .      .   23:         unsafe {
     .      .   24:             let my_num: *mut i32 = libc::malloc(1000*mem::size_of::<i32>() as libc::size_t) as *mut i32;
     .      .   25:             if my_num.is_null() {
     .      .   26:                 panic!("failed to allocate memory");
     .      .   27:             }
---
     .      .   28:         }
     .      .   29:     }
     .      .   30: }

(jeprof) disasm main
Total: 40960 B
ROUTINE ====================== hello::main
 40960  40960 B (flat, cumulative) 100.0% of total
-------------------- /home/sol/src/hello/src/main.rs
     .      .    15:      .      .        ff10: sub    $0x68,%rsp
     .      .    17: fn main()
     .      .        ff14: movl   $0x0,0x10(%rsp)
     .      .        ff1c: movl   $0xa,0x14(%rsp)
     .      .        ff24: mov    0x10(%rsp),%edi
     .      .        ff28: mov    0x14(%rsp),%esi
     .      .        ff2c: callq  f800 <<I as core::iter::traits::collect::IntoIterator>::into_iter>
     .      .        ff31: mov    %eax,0x18(%rsp)
     .      .        ff35: mov    %edx,0x1c(%rsp)
     .      .        ff39: lea    0x18(%rsp),%rdi
     .      .        ff3e: callq  f7f0 <core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::Range>::next>
     .      .        ff43: mov    %edx,0x24(%rsp)
     .      .        ff47: mov    %eax,0x20(%rsp)
     .      .        ff4b: mov    0x20(%rsp),%eax
     .      .        ff4f: cmp    $0x0,%rax
     .      .        ff53: jne    ff5a <hello::main+0x4a>
     .      .    27: }
     .      .        ff55: add    $0x68,%rsp
     .      .        ff59: retq
     .      .    17: fn main()
     .      .        ff5a: mov    0x24(%rsp),%eax
     .      .        ff5e: mov    %eax,0x5c(%rsp)
 40960  40960    21:      .      .        ff62: mov    $0x3e8,%eax
     .      .        ff67: mov    $0x4,%ecx
     .      .        ff6c: mul    %rcx
     .      .        ff6f: mov    %rax,0x8(%rsp)
     .      .        ff74: seto   %al
     .      .        ff77: test   $0x1,%al
     .      .        ff79: jne    ff99 <hello::main+0x89>
     .      .        ff7b: mov    0x8(%rsp),%rdi
 40960  40960        ff80: callq  *0x46eb1a(%rip)        # 47eaa0 <_GLOBAL_OFFSET_TABLE_+0x368>
     .      .        ff86: mov    %rax,%rdi
     .      .        ff89: mov    %rdi,0x60(%rsp)
     .      .    22: unsafe {
     .      .        ff8e: callq  f8a0 <core::ptr::mut_ptr::<impl *mut T>::is_null>
     .      .        ff93: test   $0x1,%al
     .      .        ff95: jne    ffb7 <hello::main+0xa7>
     .      .        ff97: jmp    ff39 <hello::main+0x29>
     .      .    21:      .      .        ff99: lea    0x41c190(%rip),%rdi        # 42c130 <str.0>
     .      .        ffa0: lea    0x46c089(%rip),%rdx        # 47c030 <_rjem_je_witness_depth_error+0xf0>
     .      .        ffa7: lea    -0xe3e(%rip),%rax        # f170 <core::panicking::panic>
     .      .        ffae: mov    $0x21,%esi
     .      .        ffb3: callq  *%rax
     .      .        ffb5: ud2
     .      .    23: let my_num: *mut i32 = libc::malloc(1000*mem::size_of::<i32>() as libc::size_t) as *mut i32;
     .      .        ffb7: lea    0x46c08a(%rip),%rsi        # 47c048 <_rjem_je_witness_depth_error+0x108>
     .      .        ffbe: lea    0x28(%rsp),%rdi
     .      .        ffc3: mov    %rdi,(%rsp)
     .      .        ffc7: mov    $0x1,%edx
     .      .        ffcc: callq  fe20 <core::fmt::Arguments::new_const>
     .      .        ffd1: mov    (%rsp),%rdi
     .      .        ffd5: lea    0x46c07c(%rip),%rsi        # 47c058 <_rjem_je_witness_depth_error+0x118>
     .      .        ffdc: lea    -0xef3(%rip),%rax        # f0f0 <core::panicking::panic_fmt>
     .      .        ffe3: callq  *%rax
     .      .        ffe5: ud2
     .      .        ffe7: nopw   0x0(%rax,%rax,1)
ROUTINE ====================== main

```
