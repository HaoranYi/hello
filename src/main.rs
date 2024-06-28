// cargo build --features tikv-jemallocator/profiling
//
// MALLOC_CONF=prof_leak:true,lg_prof_sample:0,prof_final:true target/debug/hello
//
// jeprof --show_bytes target/debug/hello jeprof.397079.0.f.heap 


#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

extern crate libc; // 0.2.65

use std::mem;


fn main()
{
   for _i in 0..10 {
        //println!("allocate ...");
        
        unsafe {
            let my_num: *mut i32 = libc::malloc(1000*mem::size_of::<i32>() as libc::size_t) as *mut i32;
            if my_num.is_null() {
                panic!("failed to allocate memory");
            }
        }
    }
}
