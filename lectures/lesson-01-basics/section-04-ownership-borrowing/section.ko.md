# 레슨 01 · 섹션 04 — 소유권과 차용(참조)

[English](section.md)

요약
- 데이터를 불변/가변으로 빌리고, 수명(Lifetime)을 가진 참조를 반환합니다.

학습 목표
- 소유권을 넘기지 않고 `&mut`로 데이터 수정
- `&str`로 읽고 `Option` 반환
- 입력 중 하나에서 빌린 참조를 반환할 때 명시적 라이프타임 사용

사전 지식
- 변수, 함수, 기본 제어 흐름

타임박스
- 15–20분

가이드
- 소유권: `String` 같은 값은 힙 데이터를 소유합니다. 이동(move)하면 소유권이 이전됩니다. 참조로 빌리면 소유권 없이 접근할 수 있습니다.
- 불변 참조 `&T`는 읽기 전용, 가변 참조 `&mut T`는 배타적 수정이 가능합니다.
- `String` vs `&str`:
  - `String`은 소유/가변 버퍼이며 `push_str`로 제자리 수정이 가능합니다.
  - `&str`는 UTF‑8 데이터에 대한 읽기 전용 뷰입니다.
- 라이프타임: 반환 참조가 입력에 묶여 있다면 컴파일러가 한쪽으로 추론할 수 없을 때 명시적으로 표기합니다:
  - `fn longer<'a>(a: &'a str, b: &'a str) -> &'a str`
- 문자 순회: `s.chars().next()`는 `Option<char>`를 반환하며, 비어 있으면 `None`입니다.

Option 요약(Some/None)
- 값이 있을 수도 없을 수도 있을 때 `Option<T>`를 사용합니다.
- `Some(value)`는 값을 감싸고, 없으면 `None`을 사용합니다.
- `first_char`는 그냥 `s.chars().next()`를 반환하면 됩니다.
- 명시적 매치를 원하면: `match s.chars().next() { Some(c) => Some(c), None => None }`.

유용한 내장 메서드
- `String`: `push_str`, `push`, `len`, `is_empty`, `clear`
- `&str`: `len`, `is_empty`, `starts_with`, `ends_with`, `contains`, `split_whitespace`

빠른 데모
```rust
fn append_in_place(s: &mut String, suffix: &str) { s.push_str(suffix); }
fn first_char(s: &str) -> Option<char> { s.chars().next() }
fn longer<'a>(a: &'a str, b: &'a str) -> &'a str { if a.len() >= b.len() { a } else { b } }

fn main() {
    let mut s = String::from("hi");
    append_in_place(&mut s, "!");
    println!("{} {:?}", s, first_char(&s));
    println!("{}", longer("abc", "rust"));
}
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

시작 코드
- `append_in_place`, `first_char`, `longer`를 구현하세요.

검증
- 실행: `cargo test -p lesson01_section04_ownership_borrowing`

확장(선택)
- `shorter(a, b)`를 만들어 반대 논리를 적용해 보세요.

