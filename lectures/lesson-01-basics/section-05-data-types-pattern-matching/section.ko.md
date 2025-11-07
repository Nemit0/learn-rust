# 레슨 01 · 섹션 05 — 자료형과 패턴 매칭

[English](section.md)

요약
- 간단한 열거형과 튜플 구조체를 만들고, `match`와 가드로 분기합니다.

학습 목표
- 정수 입력을 안전하게 열거형으로 매핑
- 가드가 있는 `match`와 구조 분해 패턴 사용
- `Option`으로 실패 가능 연산 표현

사전 지식
- 소유권/차용 기본

타임박스
- 15–20분

가이드
- 열거형은 관련된 변형을 하나의 타입으로 묶습니다: `enum Weekday { Mon, Tue, ... }`
- `Option<T>`는 값의 존재/부재를 표현합니다: `Some(T)` 또는 `None`.
- 튜플 구조체: `struct Point(i32, i32);` (이름 없는 필드)
- `match`에서 구조 분해와 가드를 함께 사용할 수 있습니다:
  - `match p { Point(x, y) if x == 0 || y == 0 => "axis", ... }`
- `safe_div(a, b)`는 0으로 나눌 때 패닉 대신 `None`을 반환하세요.

빠른 데모
```rust
#[derive(Debug)]
enum Weekday { Mon, Tue, Wed, Thu, Fri, Sat, Sun }

fn weekday_from(n: u8) -> Option<Weekday> {
    match n {
        1 => Some(Weekday::Mon), 2 => Some(Weekday::Tue), 3 => Some(Weekday::Wed),
        4 => Some(Weekday::Thu), 5 => Some(Weekday::Fri), 6 => Some(Weekday::Sat),
        7 => Some(Weekday::Sun), _ => None
    }
}

#[derive(Debug)]
struct Point(i32, i32);

fn quadrant(p: Point) -> &'static str {
    let Point(x, y) = p;
    if x == 0 || y == 0 { "axis" }
    else if x > 0 && y > 0 { "Q1" }
    else if x < 0 && y > 0 { "Q2" }
    else if x < 0 && y < 0 { "Q3" } else { "Q4" }
}

fn safe_div(a: i32, b: i32) -> Option<i32> { if b == 0 { None } else { Some(a / b) } }
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

시작 코드
- `weekday_from`, `quadrant`, `safe_div`를 구현하세요.

검증
- 실행: `cargo test -p lesson01_section05_data_types_pattern_matching`

확장(선택)
- `Weekday`에 `display` 함수를 `match`로 구현해 보세요.

