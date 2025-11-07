# 레슨 01 · 섹션 08 — 오류 처리

[English](section.md)

요약
- `Result`와 `?` 연산자로 오류를 반환/전파합니다.

학습 목표
- `Option`/파싱 오류를 `Result`로 변환
- `?`로 에러를 호출자에게 전파
- 상황에 맞는 사용자 친화적 오류 메시지 반환

사전 지식
- 컬렉션과 이터레이터 기초

타임박스
- 10–15분

가이드
- `Result<T, E>`는 성공(`Ok(T)`) 또는 실패(`Err(E)`)를 표현합니다.
- `?`는 `Err`를 만나면 같은 에러로 즉시 반환합니다.
- 문자열 파싱: `let n: i32 = a.parse()?;` 는 `Result<i32, ParseIntError>`를 반환합니다.
- `Option`을 `Result`로: `xs.first().copied().ok_or("empty slice".to_string())`.
- 음수 처리: `Err(format!("{} is negative", n))` 처럼 메시지를 담아 반환합니다.

빠른 데모
```rust
use std::num::ParseIntError;

fn parse_and_add(a: &str, b: &str) -> Result<i32, ParseIntError> {
    Ok(a.parse::<i32>()? + b.parse::<i32>()?)
}

fn to_positive(n: i32) -> Result<u32, String> { if n >= 0 { Ok(n as u32) } else { Err(format!("{} is negative", n)) } }
fn first_or_err(xs: &[i32]) -> Result<i32, String> { xs.first().copied().ok_or("empty slice".to_string()) }
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

검증
- 실행: `cargo test -p lesson01_section08_error_handling`

확장(선택)
- 단순한 오류 enum을 만들어 `Empty` vs `Negative`를 구분해 보세요.

