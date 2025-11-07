# 레슨 01 · 섹션 02 — 함수

[English](section.md)

목표
- 함수를 작성하고 값을 반환하는 방법을 연습합니다.

가이드
- 함수 문법: `fn name(param: Type, ...) -> ReturnType { ... }`
- 본문 마지막 식(세미콜론 없음)은 암시적으로 반환됩니다. 필요하면 `return`으로 조기 반환하세요.
- 문자열 만들기:
  - `format!("Hello, {}!", name)`은 `String`을 만듭니다.
  - `to_string()`은 `&str`를 `String`으로 변환합니다.
- 입력을 읽기만 한다면 `&str`로 빌려 오고, 새 데이터를 만들 때는 `String`을 소유하여 반환하세요.

빠른 데모
```rust
fn add(a: i32, b: i32) -> i32 { a + b }
fn greet(name: &str) -> String { format!("Hello, {}!", name) }

fn main() {
    println!("{}", add(2, 3));
    println!("{}", greet("Rust"));
}
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

문자열 치트시트
- 생성: `String::from("hi")`, `format!("{}-{}", a, b)` → `String`
- 변환: `"hi".to_string()`, `some_string.as_str()`
- 조회: `s.len()`, `s.is_empty()`

과제
1) `add(a, b)`가 두 수의 합을 반환하도록 구현하세요.
2) `greet(name)`이 `"Hello, {name}!"`를 반환하도록 구현하세요.

실행 방법
- 섹션 테스트: `cargo test -p lesson01_section02_functions`
- 전체 채점: `./scripts/grade.ps1` 또는 `bash ./scripts/grade.sh`

참고
- 정답 예시는 `answer.rs`에 있습니다(컴파일하지 않음).

