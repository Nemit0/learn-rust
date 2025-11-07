# 레슨 03 · 섹션 01 — Unsafe 기초(로우 포인터)

[English](section.md)

요약
- 로우 포인터를 통해 메모리를 읽고, 수동으로 슬라이스를 구성하는 작업을 최소한의 `unsafe` 블록으로 수행합니다.

학습 목표
- 안전성 계약을 문서화하고 `unsafe`를 작은 범위로 제한
- `*const T`/`*mut T`를 `from_raw_parts(_mut)`로 슬라이스로 변환
- 겹치지 않는 가변 슬라이스로 `split_at_mut` 재구성

사전 지식
- 소유권/차용, 슬라이스, 이터레이터 기초

타임박스
- 15–20분

가이드
- `unsafe`는 컴파일러가 검사할 수 없는 연산(로우 포인터 역참조, `extern` 호출, 가변 정적 접근, 유니온 필드 접근 등)을 허용합니다.
- `unsafe` 블록은 작게 유지하고, 전제조건(유효한 포인터, 길이 범위, 비중복 등)을 주석으로 명확히 남기세요.
- 로우 파츠에서 슬라이스 만들기:
  - `let xs = unsafe { std::slice::from_raw_parts(ptr, len) };`
  - 가변 슬라이스는 `from_raw_parts_mut(ptr, len)`

빠른 데모
```rust
use std::slice;

unsafe fn sum_via_ptr(ptr: *const i32, len: usize) -> i32 {
    let xs = slice::from_raw_parts(ptr, len);
    xs.iter().copied().sum()
}

fn main() {
    let v = vec![1,2,3,4];
    let total = unsafe { sum_via_ptr(v.as_ptr(), v.len()) };
    println!("sum={}", total);
}
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

시작 코드
- `sum_via_ptr`, `sum_slice_unsafe`, `split_at_mut_via_ptr`를 구현하세요.

검증
- 실행: `cargo test -p lesson03_section01_unsafe_basics`

흔한 실수
- `mid <= xs.len()` 확인 누락
- 겹치는 가변 슬라이스 생성 — 항상 비중복 범위만 생성하세요.

