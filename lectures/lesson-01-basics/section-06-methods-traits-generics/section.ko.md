# 레슨 01 · 섹션 06 — 메서드, 트레이트, 제네릭

[English](section.md)

요약
- 메서드를 구현하고, 간단한 트레이트를 정의/구현하며, 제네릭 함수를 작성합니다.

학습 목표
- `impl` 블록에서 `&self`/`&mut self`를 사용한 메서드 작성
- `String`을 반환하는 간단한 트레이트 구현 및 사용
- `Ord + Copy` 제약이 있는 제네릭 함수 작성

사전 지식
- 구조체/열거형, 패턴 매칭

타임박스
- 15–20분

가이드
- 메서드는 `impl Type { ... }`에 정의하며 수신자를 빌립니다.
- 면적 예시: `fn area(&self) -> u32 { self.width * self.height }`
- 트레이트는 공통 동작을 정의하며 `impl Trait for Type`으로 구현합니다.
- 제네릭과 바운드:
  - `fn max_of<T: Ord + Copy>(a: T, b: T) -> T { if a >= b { a } else { b } }`

빠른 데모
```rust
#[derive(Debug)]
struct Rectangle { width: u32, height: u32 }
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
    fn can_hold(&self, other: &Rectangle) -> bool { self.width > other.width && self.height > other.height }
}

trait Summarize { fn summary(&self) -> String; }
struct Person { name: String }
impl Summarize for Person { fn summary(&self) -> String { format!("{} says hi", self.name) } }

fn max_of<T: Ord + Copy>(a: T, b: T) -> T { if a >= b { a } else { b } }
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

검증
- 실행: `cargo test -p lesson01_section06_methods_traits_generics`

확장(선택)
- `Summarize`에 기본 메서드를 추가한 뒤 `Person`에서 재정의해 보세요.

