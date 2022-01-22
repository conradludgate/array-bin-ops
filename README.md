# array_bin_ops

An example implementation of Array Element-Wise Binary Operations in Rust.

Trying to produce efficient code where possible, avoiding any memory safety issues.
Current benchmarks show it being faster than any safe code currently available (using std only)

## Example ASM

Given the following rust code

```rust
pub fn add_i64x32(lhs: [i64; 32], rhs: [i64; 32]) -> [i64; 32] {
    Array(lhs) + rhs
}
```

It outputs the following asm, which is performing 16 i64x2 add operations, in an unrolled loop to avoid branching.

```asm
add_i64x32:
 sub     rsp, 72
 mov     rax, rdi
 movdqu  xmm1, xmmword, ptr, [rsi]
 movdqu  xmm3, xmmword, ptr, [rsi, +, 16]
 movdqu  xmm5, xmmword, ptr, [rsi, +, 32]
 movdqu  xmm7, xmmword, ptr, [rsi, +, 48]
 movdqu  xmm15, xmmword, ptr, [rsi, +, 64]
 movdqu  xmm8, xmmword, ptr, [rsi, +, 80]
 movdqu  xmm9, xmmword, ptr, [rsi, +, 96]
 movdqu  xmm10, xmmword, ptr, [rsi, +, 112]
 movdqu  xmm14, xmmword, ptr, [rsi, +, 128]
 movdqu  xmm13, xmmword, ptr, [rsi, +, 144]
 movdqu  xmm12, xmmword, ptr, [rsi, +, 160]
 movdqu  xmm11, xmmword, ptr, [rsi, +, 176]
 movups  xmm0, xmmword, ptr, [rsi, +, 192]
 movaps  xmmword, ptr, [rsp], xmm0
 movdqu  xmm2, xmmword, ptr, [rsi, +, 208]
 movups  xmm0, xmmword, ptr, [rsi, +, 224]
 movaps  xmmword, ptr, [rsp, +, 48], xmm0
 movdqu  xmm0, xmmword, ptr, [rdx]
 paddq   xmm0, xmm1
 movdqa  xmmword, ptr, [rsp, +, 32], xmm0
 movdqu  xmm0, xmmword, ptr, [rdx, +, 16]
 paddq   xmm0, xmm3
 movdqa  xmmword, ptr, [rsp, +, 16], xmm0
 movdqu  xmm4, xmmword, ptr, [rdx, +, 32]
 paddq   xmm4, xmm5
 movdqu  xmm6, xmmword, ptr, [rdx, +, 48]
 paddq   xmm6, xmm7
 movdqu  xmm1, xmmword, ptr, [rdx, +, 64]
 paddq   xmm1, xmm15
 movdqu  xmm15, xmmword, ptr, [rdx, +, 80]
 paddq   xmm15, xmm8
 movdqu  xmm8, xmmword, ptr, [rdx, +, 96]
 paddq   xmm8, xmm9
 movdqu  xmm9, xmmword, ptr, [rdx, +, 112]
 paddq   xmm9, xmm10
 movdqu  xmm10, xmmword, ptr, [rdx, +, 128]
 paddq   xmm10, xmm14
 movdqu  xmm14, xmmword, ptr, [rdx, +, 144]
 paddq   xmm14, xmm13
 movdqu  xmm13, xmmword, ptr, [rdx, +, 160]
 paddq   xmm13, xmm12
 movdqu  xmm12, xmmword, ptr, [rdx, +, 176]
 paddq   xmm12, xmm11
 movdqu  xmm3, xmmword, ptr, [rdx, +, 192]
 paddq   xmm3, xmmword, ptr, [rsp]
 movdqu  xmm7, xmmword, ptr, [rdx, +, 208]
 paddq   xmm7, xmm2
 movdqu  xmm5, xmmword, ptr, [rdx, +, 224]
 paddq   xmm5, xmmword, ptr, [rsp, +, 48]
 movdqu  xmm11, xmmword, ptr, [rsi, +, 240]
 movdqu  xmm0, xmmword, ptr, [rdx, +, 240]
 paddq   xmm0, xmm11
 movaps  xmm2, xmmword, ptr, [rsp, +, 32]
 movups  xmmword, ptr, [rdi], xmm2
 movaps  xmm2, xmmword, ptr, [rsp, +, 16]
 movups  xmmword, ptr, [rdi, +, 16], xmm2
 movdqu  xmmword, ptr, [rdi, +, 32], xmm4
 movdqu  xmmword, ptr, [rdi, +, 48], xmm6
 movdqu  xmmword, ptr, [rdi, +, 64], xmm1
 movdqu  xmmword, ptr, [rdi, +, 80], xmm15
 movdqu  xmmword, ptr, [rdi, +, 96], xmm8
 movdqu  xmmword, ptr, [rdi, +, 112], xmm9
 movdqu  xmmword, ptr, [rdi, +, 128], xmm10
 movdqu  xmmword, ptr, [rdi, +, 144], xmm14
 movdqu  xmmword, ptr, [rdi, +, 160], xmm13
 movdqu  xmmword, ptr, [rdi, +, 176], xmm12
 movdqu  xmmword, ptr, [rdi, +, 192], xmm3
 movdqu  xmmword, ptr, [rdi, +, 208], xmm7
 movdqu  xmmword, ptr, [rdi, +, 224], xmm5
 movdqu  xmmword, ptr, [rdi, +, 240], xmm0
 add     rsp, 72
 ret
```
