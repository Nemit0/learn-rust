# 레슨 01 · 섹션 03 — 제어 흐름

[English](section.md)

요약
- `if`/`match`로 분기하고 간단한 반복문을 연습합니다.

학습 목표
- 올바른 순서와 가드로 조건 분기 구현
- `for` 루프로 누적 계산 수행

사전 지식
- 변수, 함수

타임박스
- 10–15분

가이드
- `if`는 식입니다: `let label = if n % 2 == 0 { "even" } else { "odd" };`
- `match`는 위에서 아래로 패턴을 비교하고, 처음 일치하는 분기가 선택됩니다.
- 가드로 분기를 정교화할 수 있습니다: `match n { x if x % 15 == 0 => ... }`.
- 반복문:
  - `for i in 1..=n { ... }` 는 포함 범위 순회
  - `while cond { ... }` 는 조건이 거짓이 될 때까지 반복
- FizzBuzz는 15를 먼저 검사해야 3/5가 먼저 매치되는 오류를 피할 수 있습니다.

빠른 데모
```rust
fn fizz_buzz(n: i32) -> String {
    if n % 15 == 0 { "FizzBuzz".into() }
    else if n % 3 == 0 { "Fizz".into() }
    else if n % 5 == 0 { "Buzz".into() }
    else { n.to_string() }
}

fn main() { for n in 1..=6 { print!("{} ", fizz_buzz(n)); } }
```

실행
- Windows (PowerShell): `rustc -O demo.rs -o demo.exe; .\demo.exe`
- macOS/Linux: `rustc -O demo.rs -o demo && ./demo`

시작 코드
- `src/lib.rs`의 `is_even`, `fizz_buzz`, `sum_up_to`를 구현하세요.

과제
1) `is_even(n)`은 `n`이 2로 나눠떨어지면 `true` 반환
2) `fizz_buzz(n)`은 표준 규칙 준수(15 먼저)
3) `sum_up_to(n)`은 1..=n 합 계산(0이면 0)

힌트
- FizzBuzz에서 문자열 리터럴은 그대로 사용하고, 필요 시 `to_string()`을 쓰면 됩니다.

흔한 실수
- 3이나 5를 15보다 먼저 검사해 15에서 오답이 나오는 경우

검증
- 실행: `cargo test -p lesson01_section03_control_flow`

확장(선택)
- `Iterator::sum`으로 `sum_up_to`를 다시 구현해 비교해 보세요.

