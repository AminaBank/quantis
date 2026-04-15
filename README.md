**Problem-1: The Oracle Linux UEK 6.12 kernel (the el10uek variant) has a stricter GCC configuration than the standard el10_1 kernel — specifically, it promotes -Wempty-body to -Werror=empty-body, turning the warning into a build error.**

The cause is in include/xdma-core.h lines 188–196. When XDMA_DEBUG == 0 (the normal build mode), all the debug macros expand to nothing:
In xdma-core.c, these macros are used as the sole statement in if/else branches (e.g. line 578-580, 988-990, 2841-2842, etc.). When the macro expands to nothing, the compiler sees an empty if/else body and complains.
Change all 8 null macro definitions to use do { } while(0) — the standard C idiom for safe no-op macros that are harmless to the compiler:

**Correction: In drivers/src/module/include/xdma-core.h - change the 8 null debug macro definitions to do { } while(0)**


```
189,196c189,196
< #define dbg_desc(...)
< #define dbg_io(...)
< #define dbg_fops(...)
< #define dbg_perf(fmt, ...)
< #define dbg_sg(...)
< #define dbg_tfr(...)
< #define dbg_irq(...)
< #define dbg_init(...)
---
> #define dbg_desc(...)      do { } while(0)
> #define dbg_io(...)        do { } while(0)
> #define dbg_fops(...)      do { } while(0)
> #define dbg_perf(fmt, ...) do { } while(0)
> #define dbg_sg(...)        do { } while(0)
> #define dbg_tfr(...)       do { } while(0)
> #define dbg_irq(...)       do { } while(0)
> #define dbg_init(...)      do { } while(0)
```




**Problem-2: ‘make install’ fails after ‘make’ as ../bin directory already exists**

make install fails

**Correction: In drivers/src/module/Makefile – only create directory if not exist**

```
31c31
<       mkdir ../bin
---
>       mkdir -p ../bin
```


**Updated rpm package is provided as pcie-chip-unix-dkms-24.4-3.el10.x86_64.rpm**

Note: this is obviously not signed with IDQuantique GPG key, so install via 

```
rpm -e --allmatches pcie-chip-unix-dkms   #remove old against all kernels
rpm -i --nosignature  drivers/rpm/pcie-chip-unix-dkms-24.4-3.el10.x86_64.rpm   #install new
```

Or simplye *make* then *make install* on src directory.
