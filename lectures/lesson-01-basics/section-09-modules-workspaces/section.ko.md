# 레슨 01 · 섹션 09 — 모듈, 크레이트, 워크스페이스

[English](section.md)

요약
- 코드를 모듈로 구성하고, 크레이트 루트에서 필요한 API만 재노출합니다.

학습 목표
- `pub mod`와 `pub` 함수 선언
- `pub use`로 선택적 재노출

사전 지식
- 오류 처리, 컬렉션/이터레이터

타임박스
- 10–15분

가이드
- 모듈 예시: `pub mod math { pub fn add(...) { ... } }`
- 기본은 비공개이므로 공개할 항목에만 `pub`을 붙이세요.
- 크레이트 루트에서 `pub use`로 학생이 사용할 최소 표면을 재노출합니다: `pub use math::add;`
- 경로 사용: `crate::`, `super::`, 모듈 경로

빠른 데모
```rust
mod math { pub fn add(a: i32, b: i32) -> i32 { a + b } pub fn mul(a: i32, b: i32) -> i32 { a * b } }
mod string_utils { pub fn shout(s: &str) -> String { format!("{}!", s.to_uppercase()) } }
pub use math::{add, mul};
pub use string_utils::shout;
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

검증
- 실행: `cargo test -p lesson01_section09_modules_workspaces`

확장(선택)
- 비공개 헬퍼 함수를 만들고 공개 함수에서 사용해 보세요.

